# llm_rust_django_backend
    Backend deployment on heroku for repo: llm_rust_django_nextjs

## Runtime: 
   * https://devcenter.heroku.com/articles/python-support#supported-runtimes
   * heroku config:set DISABLE_COLLECTSTATIC=1
   * Env keys added to config.
   * heroku logs -n 200

## Deployment: 
   * https://llmrustdjangobackend-5203aa413008.herokuapp.com/api/docs#


## Further Developments Done: 
   Avoid Hallucination: Use case for high tech companies in critical data sector.
   Why highly profile companies or legal tech companies need to avoid hallucination
    or misplaced texts ? Why do the chatbots need to be within the censorship confinement?
    
   Example: Google shares drop by 100 million because of misplaced
      information from 

    Flowapp: Reduces time footprint from coversation to solution. (pitch this)
    Call to setup of pipelines for a given customer directly from video.
    Good support.
    
## Solution: 
    * Prompt Engineering, Reinforcement Learning Human Feedback (RLHF):
        -- Pros and Cons: Prompt engineering requires lots of effort to build
           from expensive human correction and annotation. One way to get around that
           is to have a series of defined templates which define the category of those topics
           Use GPT or any general large language model to build variations of those templates.
           
           As new prompts from user come in, classify them using a vector database.

    * Guard Rails: Developed by Nvidia: add a layer of guard onto the 
      automation software.

## The Hallucination can also be reduced using this here: 
https://colab.research.google.com/drive/1AXccYmC11kWJ8zZtqS78ZvmBCC7yIPI0#scrollTo=yjs-uPXBrnQs

## Goal of This Backend:
   * Supabase
   * Rust, No Third Party Applications connected to it.
   * Prove hallucination solved

## API Endpoints:
   * pass in .pdfs inside: get the embeddings.

## Pitch on Tech:
   * Rust Backend, end to end solutions. 
     User friendly API
     user privacy
     No hallucination from the AI Agent.

## Pitch on Product:
   * Netflix Subscription churn prediction, automate sending personalised mails. a/B testing
   * Find, contact, and close your ideal buyers with over 265M contacts and streamlined engagement workflows powered by AI. 

    * Give a voice to the data of company.
    * Sales Intelligence: figuring out why sales is so poor.
    * Talk to Sales Data, Churn prediction. 
    * Small Startups --> Get their leads.

    * Big Companies --> Really good customer service.
    * Companies do not have the expertise to find new sales lead. 

## Builders Build. We help you sell from day 1 so you can focus on building.
   -- Sales Intelligence.
       -- send leads. Inbound and outbound.
       -- 
   -- Legal matters: we help you avoid large corporate fines, keep company in standing order, finance analysis.
   -- Draft Letters: 
   -- Test the speed of inference: https://www.youtube.com/watch?v=h6qZM76eOYE
   -- 

## Manager: 
   * Langchain-rs: rust based langchain 
   * Embeddings: Loaders for the documents.
   * LLaMa or OpenAI  
   * Sequence of OpenAI loaders:
   * Get the Data with the Embeddings: 
   * Close Solution: No third party apis from other providers.
   * https://github.com/gaxler/llama2.rs, the rust version of this company.

## Technical Performance analysis of inference
   140mb: model fast inference.

## Analysis of the code:
    https://colab.research.google.com/drive/1t3NRHDjs25jS5tgpmSdlw1gGw9HVQRVZ#scrollTo=GpGBT-_SYOJl

## Ways of Working:
   OpenAI Code Interpreter: https://github.com/kesor/chatgpt-code-plugin --> code chatgpt-code plugin (install it!)

## Sell the Software Even from beforehand (super important) (Door to Door to Companies online)

##  What makes a great Company?
    -- Efficient operation: Sales Intelligence B2B SaaS Prospecting.
    -- Now it does not stop here: we help leverage all of your tools in one stop shop 
       for automation. 
    -- Replace tools like UI Path, Zapier. 
    -- video automation for sales: Customer interaction. (similar to parloa).
    -- get the engagement rate increase
    -- a lot of integrations (emails, callings, infinite customisation on pipelines)
       --> lowest level personalised emails, approaches.
    -- video for instructing videos --> get the automated pipelines 
    -- set up for them. (show it to customers and test).

## Nemo Guardrails:
   DONE: Test on Google Colab
   TODO: add new endpoint on Django Ninja
   TODO: connect with the Form Input on the Frontend.
 
   * Nvidia_Guardrails:  
     The Guardrails provide a layer on top of existing large language models in order to eliminate toxicity
     and prevent politically incorrect answers. Example: Google shares dropped down by 100Billion (total)
     after an error with BARD.
      
     I give some examples here: https://colab.research.google.com/drive/1Ih7VluVb9L4G7ysekrr7MqCDCHWYrksN#scrollTo=6KbZRU2XcKl9
      
     The Vector DB:https://app.pinecone.io/organizations/-NbuWN70ojQ-hVtevL00/projects/asia-southeast1-gcp-free:9ed6d56/keys
   
      Input Data: .pdf format 
      The RAG with Guardrail allows to reduce Hallucination by providing a highly easily customisable 

## References:
   * Frontend: https://llm-backend-rust.vercel.app/
   * Backend:  Deployment of Django on Vercel.json: https://www.youtube.com/watch?v=ZjVzHcXCeMU&t=423s
   * Backend: also on heroku: https://llmrustdjangobackend-5203aa413008.herokuapp.com/api/docs#



## Further Developments Done: 
   Avoid Hallucination: Use case for high tech companies in critical data sector.
   Why highly profile companies or legal tech companies need to avoid hallucination
    or misplaced texts ? Why do the chatbots need to be within the censorship confinement?
    
   Example: Google shares drop by 100 million because of misplaced
      information from 

## Solution: 
    * Prompt Engineering, Reinforcement Learning Human Feedback (RLHF):
        -- Pros and Cons: Prompt engineering requires lots of effort to build
           from expensive human correction and annotation. One way to get around that
           is to have a series of defined templates which define the category of those topics
           Use GPT or any general large language model to build variations of those templates.
           
           As new prompts from user come in, classify them using a vector database.

    * Guard Rails: Developed by Nvidia: add a layer of guard onto the 
      automation software.

## The Hallucination can also be reduced using this here: 
https://colab.research.google.com/drive/1AXccYmC11kWJ8zZtqS78ZvmBCC7yIPI0#scrollTo=yjs-uPXBrnQs

## Goal of This Backend:
   * Supabase
   * Rust, No Third Party Applications connected to it.
   * Prove hallucination solved

## API Endpoints:
   * pass in .pdfs inside: get the embeddings.

## Pitch on Tech:
   * Rust Backend, end to end solutions. 
     User friendly API
     user privacy
     No hallucination from the AI Agent.

## Pitch on Product:
   * Netflix Subscription churn prediction, automate sending personalised mails. a/B testing
   * Find, contact, and close your ideal buyers with over 265M contacts and streamlined engagement workflows powered by AI. 

    * Give a voice to the data of company.
    * Sales Intelligence: figuring out why sales is so poor.
    * Talk to Sales Data, Churn prediction. 
    * Small Startups --> Get their leads.

    * Big Companies --> Really good customer service.
    * Companies do not have the expertise to find new sales lead. 

## Builders Build. We help you sell from day 1 so you can focus on building.
   -- Sales Intelligence.
       -- send leads. Inbound and outbound.
       -- 
   -- Legal matters: we help you avoid large corporate fines, keep company in standing order, finance analysis.
   -- Draft Letters: 
   -- Test the speed of inference: https://www.youtube.com/watch?v=h6qZM76eOYE
   -- 

## Manager: 
   * Langchain-rs: rust based langchain 
   * Embeddings: Loaders for the documents.
   * LLaMa or OpenAI  
   * Sequence of OpenAI loaders:
   * Get the Data with the Embeddings: 
   * Close Solution: No third party apis from other providers.
   * https://github.com/gaxler/llama2.rs, the rust version of this company.

## Technical Performance analysis of inference
   140mb: model fast inference.

## Analysis of the code:
    https://colab.research.google.com/drive/1t3NRHDjs25jS5tgpmSdlw1gGw9HVQRVZ#scrollTo=GpGBT-_SYOJl

## Ways of Working:
   OpenAI Code Interpreter: https://github.com/kesor/chatgpt-code-plugin --> code chatgpt-code plugin (install it!)

## Sell the Software Even from beforehand (super important) (Door to Door to Companies online)

##  What makes a great Company?
    -- Efficient operation: Sales Intelligence B2B SaaS Prospecting.
    -- Now it does not stop here: we help leverage all of your tools in one stop shop 
       for automation. 
    -- Replace tools like UI Path, Zapier. 
    -- video automation for sales: Customer interaction. (similar to parloa).
    -- get the engagement rate increase
    -- a lot of integrations (emails, callings, infinite customisation on pipelines)
       --> lowest level personalised emails, approaches.
    -- video for instructing videos --> get the automated pipelines 
    -- set up for them. (show it to customers and test).

## Pure C Inference for  Stories llam2 (smaller network with similar architecture to LLAMA2 of Facebeook)
   * 15M Parameters, 110M Parameters. 1GB dataset. (mini Lama2 for speed bare bone on Macbook 2 Pro.)
     Train 2.1M rows, validation 21K rows

## Nemo Guardrails:
   DONE: Test on Google Colab
   TODO: add new endpoint on Django Ninja
   TODO: connect with the Form Input on the Frontend.
 
   * Nvidia_Guardrails:  
     The Guardrails provide a layer on top of existing large language models in order to eliminate toxicity
     and prevent politically incorrect answers. Example: Google shares dropped down by 100Billion (total)
     after an error with BARD.
      
     I give some examples here: https://colab.research.google.com/drive/1Ih7VluVb9L4G7ysekrr7MqCDCHWYrksN#scrollTo=6KbZRU2XcKl9
      
     The Vector DB:https://app.pinecone.io/organizations/-NbuWN70ojQ-hVtevL00/projects/asia-southeast1-gcp-free:9ed6d56/keys
   
      Input Data: .pdf format 
      The RAG with Guardrail allows to reduce Hallucination by providing a highly easily customisable 
   
    https://colab.research.google.com/github/pinecone-io/examples/blob/master/learn/generation/chatbots/nemo-guardrails/03-rag-with-actions.ipynb#scrollTo=nTMYxfWqcKl_

## Rust_Data_Connector
    * https://crates.io/crates/rust-bert

## Move the entire pipeline to Rust

## References:
   * Frontend: https://llm-backend-rust.vercel.app/
   * Backend:  Deployment of Django on Vercel.json: https://www.youtube.com/watch?v=ZjVzHcXCeMU&t=423s
   * Backend: also on heroku: https://llmrustdjangobackend-5203aa413008.herokuapp.com/api/docs#
   * Huggingface: https://huggingface.co/datasets/roneneldan/TinyStories
   * add option to win the Guardrails or any way to improve over Toxicity


## Key Things to pitch to investors
   * Rust Reliability
   * Serve LLAMA2 on our own tech stack (end to end): No sharing  of information with openai or any other 3rd party providers
   * prevent against attacks: Guardrails
   * Every company uses lots of tools for their day to day operations. Number has kept increasing over and over again
     which tools to use and why (advice)
     custom automation made only for you. No more buggy UI path and huge consultation fees like UI Path and other RPA tools
   * https://www.merge.dev/ (inspiration)
   * Send us either a text description of your automation that want us to implement: 