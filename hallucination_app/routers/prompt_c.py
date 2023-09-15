"""
Ninja Django API to collect the
"""
from ninja import Router
import subprocess
router = Router()

#TODO: update to add the argument for model_path
from django.http import StreamingHttpResponse

import os

from django.conf import settings

print (settings.BASE_DIR)

#Add path to
# Add path to
@router.get("/prompt_c")
def StoryTelling(request,input:str):
    assert (isinstance(input,str))
    #llama2_bin = "/Users/ousheshharadhun/Documents/Workspace/FacebookLLAMA/llm_rust_django_backend/manager/llama2.c/./run"
    #llama2_model = "/Users/ousheshharadhun/Documents/Workspace/FacebookLLAMA/llm_rust_django_backend/manager/llama2.c/stories15M.bin"
    binaries_dir = os.path.join(settings.BASE_DIR, "hallucination_app", "binaries")

    c_file_path = os.path.join(settings.BASE_DIR, "hallucination_app", "binaries", "run.c")

    llama2_bin = os.path.join(settings.BASE_DIR, "hallucination_app","binaries/./run")
    llama2_model = os.path.join(settings.BASE_DIR,"hallucination_app","binaries","stories15M.bin")


    # Check if the binary doesn't exist
    if not os.path.exists(llama2_bin):
        # Compile the C code
        compile_command = ["gcc", c_file_path, "-o", llama2_bin]
        subprocess.run(compile_command, check=True)

    cmd = [llama2_bin, llama2_model, "-t", "0.8", "-n", "256", "-i", input]
    process = subprocess.Popen(cmd,stdout=subprocess.PIPE,text=True,cwd=binaries_dir)

    def stream():
        for line in iter(process.stdout.readline, ""):
            yield f"data: {line}\n\n"
            #print (f"data: {line}\n\n")
        process.stdout.close()
        process.wait()
    return StreamingHttpResponse(stream(), content_type="text/event-stream")