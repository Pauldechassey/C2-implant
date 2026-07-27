import logging
import time
from contextlib import asynccontextmanager
from fastapi import FastAPI, Request, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware
from app.database.database import engine
from app.ws_manager import log_manager, command_manager

logging.basicConfig(level=logging.INFO, format="%(asctime)s | %(levelname)s | %(message)s")
logger = logging.getLogger("team-server")


@asynccontextmanager
async def lifespan(app: FastAPI):
    from app.database.database import Base
    Base.metadata.create_all(bind=engine)
    print("\n\033[92m" + "=" * 44)
    print("   TEAM SERVER")
    print("=" * 44 + "\033[0m\n")
    yield


app = FastAPI(
    title="Team Server API",
    description="Team Server API for C2 operations",
    lifespan=lifespan
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


SILENT_ROUTES = {("GET", "/commands/"), ("GET", "/agent/status")}

@app.middleware("http")
async def log_requests(request: Request, call_next):
    start = time.time()
    response = await call_next(request)
    duration = (time.time() - start) * 1000
    client = request.client.host if request.client else "unknown"
    msg = f"{client} | {request.method} {request.url.path} | {response.status_code} | {duration:.1f}ms"
    logger.info(msg)
    if (request.method, request.url.path) not in SILENT_ROUTES:
        await log_manager.broadcast(msg)
    return response


@app.websocket("/ws/logs")
async def ws_logs(ws: WebSocket):
    await log_manager.connect(ws)
    try:
        while True:
            await ws.receive_text()
    except WebSocketDisconnect:
        log_manager.disconnect(ws)


@app.websocket("/ws/commands")
async def ws_commands(ws: WebSocket):
    await command_manager.connect(ws)
    try:
        while True:
            await ws.receive_text()
    except WebSocketDisconnect:
        command_manager.disconnect(ws)


@app.get("/", tags=["root"])
def read_root():
    return {"message": "Team Server API is running"}


from app.routes.command_routes import router as command_router
from app.routes.agent_routes import router as agent_router

app.include_router(command_router, tags=["commands"])
app.include_router(agent_router)
