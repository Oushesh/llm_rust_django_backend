"""
Ninja Django API to call in the routers.
"""
import os
from ninja import Router
import subprocess
router = Router()

#TODO: update to add the argument for model_path
from django.http import StreamingHttpResponse
from django.conf import settings

@router.get("/prompt_c")
def StoryTelling(request,input:str):
    assert (isinstance(input,str))
    binaries_dir = os.path.join(settings.BASE_DIR, "agent_app", "binaries")
    llama2_bin = os.path.join(settings.BASE_DIR, "agent_app","binaries/./run")
    llama2_model = os.path.join(settings.BASE_DIR,"agent_app","binaries","stories15M.bin")

    """"
    # Check if the binary doesn't exist
    if not os.path.exists(llama2_bin):
        # Compile the C code
        compile_command = ["gcc", c_file_path, "-o", llama2_bin]
        subprocess.run(compile_command, check=True)
    """

    cmd = [llama2_bin, llama2_model, "-t", "0.8", "-n", "256", "-i", input]
    process = subprocess.Popen(cmd,stdout=subprocess.PIPE,text=True,cwd=binaries_dir)

    def stream():
        for line in iter(process.stdout.readline, ""):
            yield f"data: {line}\n\n"           #yield is a lazy loader and generator. Memory cost optimisation
        process.stdout.close()
        process.wait()
    return StreamingHttpResponse(stream(), content_type="text/event-stream")