"""
This endpoint will take in
text prompt from the user and
calls a specific function
which will perform a task.
OpenAI Endpoint decides on how
"""

#Current state: Data on GCP Bucket

import os
from dotenv import load_dotenv
import openai
from ninja import Router
import subprocess
import logging
import json
from django.http import StreamingHttpResponse
from django.conf import settings

from agent_app.utils.utils import upload_logs_to_gcs

router = Router()

#TODO: updated to add the argument for model_path


def get_variables_llm_session()->str:
    BASE_DIR = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    # print ("BASE_DIR", BASE_DIR)
    env_file  = ".env"
    log_file_path = os.path.join(BASE_DIR, "logs/session.log")

    # Configure logging
    logging.basicConfig(filename=log_file_path, level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

    dotenv_path = os.path.join(BASE_DIR, env_file)
    logging.info(".env path:",dotenv_path)
    print (dotenv_path)
    load_dotenv(dotenv_path)
    api_key = os.getenv("OPENAI_API_KEY")

    try:
        if api_key:
            info = "OpenAI Key Successfully loaded"
            print("OpenAI Key Successfully loaded")
            logging.info(info)
        else:
            error_msg = f"API Key not found in ${env_file} file"
            raise ValueError(error_msg)

    except ValueError as e:
        logging.error(str(e))

    assert isinstance(api_key,str)
    #api_key = "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"
    return api_key

@router.get("/2text")
def convert2text(request,user_prompt:str):
    assert (isinstance(user_prompt,str))
    BASE_DIR = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    local_log_path = os.path.join(BASE_DIR,"logs/session.log")

    audio_dir = os.path.join(settings.BASE_DIR,"agent_app","audio")
    audio_path = os.path.join(audio_dir,"output_audio.mp3")

    # command line input, add also the Key of openai here
    api_key = get_variables_llm_session()
    print (api_key)
    cmd = [
        "whisper-stream",
        "-f", audio_path,
        "-p", "./",
        "-l", "en",
        "-t", api_key
    ]

    process = subprocess.Popen(cmd,stdout=subprocess.PIPE,text=True)

    output, error = process.communicate()

    # Check for any errors.
    # TODO: add this to a logging system

    if process.returncode == 0:
        logging.info(f"Command executed successfully")
        logging.info(f"Output:")

        upload_logs_to_gcs(logging,local_log_path, "logs_impactnexus", "parquet_conversion/parquet_conversion.log")
        return {"output":f"Command executed successfully and output is:{output}"}
    else:
        logging.error(f"Error occurred:")
        assert isinstance(error,str)
        logging.error(error)
        upload_logs_to_gcs(logging,local_log_path, "logs_impactnexus", "parquet_conversion/parquet_conversion.log")
        return {"output":error}

# TODO: add log file system to this endpoint here and as well save the logs to GCP.
# TODO: add the transcription to postgresql database


