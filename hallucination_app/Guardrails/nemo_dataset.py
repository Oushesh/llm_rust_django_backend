# SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
# SPDX-License-Identifier: Apache-2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from typing import Any, Callable, Coroutine
from langchain.llms.base import BaseLLM
from nemoguardrails import LLMRails, RailsConfig

import yaml

COLANG_CONFIG = """
define user express greeting
  "hi"

define user express ill intent
  "I hate you"
  "I want to destroy the world"
  
define user ask politics
    "what are your political beliefs?"
    "thoughts on the president?"
    "left wing"
    "right wing"

define bot answer politics
    "I'm a shopping assistant, I don't like to talk of politics."
    "Sorry I can't talk about politics!"


define flow politics
    user ask politics
    bot answer politics
    bot offer help

define bot express cannot respond
  "I'm sorry I cannot help you with that."

define user express question
   "What is the current unemployment rate?"

# Basic guardrail example
define flow
  user express ill intent
  bot express cannot respond

# Question answering flow
define flow
  user ...
  $answer = execute llama_index_query(query=$last_user_message)
  bot $answer

"""

import yaml

from dotenv import load_dotenv

load_dotenv(".env")

from knowledge_base.constants import yaml_content, rag_colang_content
from utils import test_colang_config,test_model_config

def demo():
    try:
        import llama_index
        from llama_index.indices.query.base import BaseQueryEngine
        from llama_index.response.schema import StreamingResponse

    except ImportError:
        raise ImportError(
            "Could not import llama_index, please install it with "
            "`pip install llama_index`."
        )
    #model_config = "knowledge_base/model_config.yaml"
    #colang_config = "knowledge_base/colang.co"

    #TODO: add testing function to test the contents
    test_colang_config(rag_colang_content)
    test_model_config(yaml_content)

    # initialize rails config
    config = RailsConfig.from_content(
        colang_content=rag_colang_content,
        yaml_content=yaml_content
    )

    app = LLMRails(config)

    def _get_llama_index_query_engine(llm: BaseLLM):
        docs = llama_index.SimpleDirectoryReader(
            input_files=["knowledge_base/report.md"]
        ).load_data()
        llm_predictor = llama_index.LLMPredictor(llm=llm)
        index = llama_index.GPTVectorStoreIndex.from_documents(
            docs, llm_predictor=llm_predictor
        )
        default_query_engine = index.as_query_engine()
        return default_query_engine

    def _get_callable_query_engine(
        query_engine: BaseQueryEngine,
    ) -> Callable[[str], Coroutine[Any, Any, str]]:
        async def get_query_response(query: str) -> str:
            response = query_engine.query(query)
            if isinstance(response, StreamingResponse):
                typed_response = response.get_response()
            else:
                typed_response = response
            response_str = typed_response.response
            if response_str is None:
                return ""
            return response_str

        return get_query_response

    query_engine = _get_llama_index_query_engine(app.llm)
    app.register_action(
        _get_callable_query_engine(query_engine), name="llama_index_query"
    )

    history = [{"role": "user", "content": "Who is the president of United States?"}]
    result = app.generate(messages=history)
    print(result)


if __name__ == "__main__":
    #test_yaml_file("knowledge_base/model_config.yaml")
    demo()
    
