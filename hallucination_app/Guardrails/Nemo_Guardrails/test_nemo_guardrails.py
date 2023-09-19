from nemoguardrails import LLMRails, RailsConfig

#Step1: Database Knowledge Download (This can also be a single file in a simple use case scenario)
from datasets import load_dataset
import os
import
data = load_dataset(
    "jamescalam/llama-2-arxiv-papers-chunked",
    split="train"
)

data[0]
print (data[0])
#from examples.examples.learn.generation.chatbots.nemo_guardrails.nemo_dataset import *

## The pdf version of where you have the file is here: High-Performance Neural Networks\nfor Visual Object Classication

data = data.map(lambda x: {
    'uid': f"{x['doi']}-{x['chunk-id']}"
})
data

data = data.to_pandas()
# drop irrelevant fields
data = data[['uid', 'chunk', 'title', 'source']]

