from fastapi import APIRouter
from .. import agent_status

router = APIRouter(prefix="/agent", tags=["agent"])


@router.get("/status")
def get_status():
    return {"last_seen": agent_status.get_last_seen()}
