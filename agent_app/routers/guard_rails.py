"""
Endpoint for Nvidia nemo guard_rails
"""
import os
from ninja import Router
from django.http import StreamingHttpResponse
from agent_app.Guardrails.nemo_dataset import guardrail
router = Router()


@router.get("/guard")
def guard_rail(request,prompt:str):
    assert (isinstance(prompt,str)==True)
    return StreamingHttpResponse(guardrail(prompt))