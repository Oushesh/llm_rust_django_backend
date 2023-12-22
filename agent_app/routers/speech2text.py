"""
This endpoint will take in
text prompt from the user and
calls a specific function
which will perform a task.
OpenAI Endpoint decides on how
"""

#Current state: Data on GCP Bucket

import os
from ninja import Router
import subprocess
router = Router()

#TODO: updated to add the argument for model_path
from django.http import StreamingHttpResponse
from django.conf import settings

@router.get("/2text")
def convert2text(request,user_prompt:str):
    assert (isinstance(user_prompt,str))

    audio_dir = os.path.join(settings.BASE_DIR,"agent_app","audio")
    audio_path = os.path.join(audio_dir,"output_audio.mp3")

    # command line input, add also the Key of openai here

    cmd = [
        "whisper-stream",
        "-f", audio_path,
        "-p", "./",
        "-l", "en",
        "-t", "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"
    ]

    process = subprocess.Popen(cmd,stdout=subprocess.PIPE,text=True)

    output, error = process.communicate()

    # Check for any errors.
    # TODO: add this to a logging system

    if process.returncode == 0:
        print ("Command executed successfully.")
        print ("Output:")
        print (output)
    else:
        print ("Error occurred:")
        print (error)

    def stream():
        for line in iter(process.stdout.readline,""):
            yield f"data: {line}\n\n"
        process.stdout.close()
        process.wait()
    return StreamingHttpResponse(stream(),content_type="text/event-stream")

# TODO: add the streaming version of the http response
# TODO: add log file system to this endpoint here and as well save the logs to GCP.


