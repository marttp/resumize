# Resumize (เรซูมั้ย?)

## Overview
This application helps generate resume content based on your experience and the job description of your interest.

![CleanShot 2567-04-28 at 01 25 21@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/b04e820e-68a5-4c15-b93c-c43aad035fe6)

## Article
- [Guide for your new resume to apply the dream job with Resumize (My Rust & RAG side project)](https://tpbabparn.medium.com/guide-for-your-new-resume-to-apply-the-dream-job-with-resumize-my-rust-rag-side-project-486cce64e7ea)
- [ช่วยตัดสินใจสร้าง Resume โดยอิงจาก Job description ด้วย Resumize (My Rust & RAG side project)](https://tpbabparn.medium.com/ช่วยตัดสินใจสร้าง-resume-โดยอิงจาก-job-description-ด้วย-resumize-my-rust-rag-side-project-1f70e2ef5eeb)

## Tech Stack
- [Dioxus](https://dioxuslabs.com/) 0.5.1
- [Actix Web](https://actix.rs/) 4.5.1
- [Qdrant](https://qdrant.tech/) 1.9.0
- [LlamaEdge](https://github.com/LlamaEdge/LlamaEdge) with [Meta Llama 3](https://llama.meta.com/llama3/)

## Features
- **Generate Resume Content**: Input your dream job description and get a customized resume content. Please note that the result is not perfect.
- **Upload Experience**: Upload your past experiences in JSON format to be used for generating the resume content.

## Demo

[Video demo](https://youtu.be/fBumbjukAF8?si=uSV3iefK_bEs8au0)

[![Demo - Resumize (Dioxus, Actix Web, Qdrant, LlamaEdge)](https://img.youtube.com/vi/fBumbjukAF8/0.jpg)](https://youtu.be/fBumbjukAF8?si=uSV3iefK_bEs8au0)

## How it's designed

The project has been designed for few use-cases
1. Upload my own experience (as JSON)
2. Upload Job description from recruiter by copy from job market
3. Create Resume content based on job description with suggestion

![CleanShot 2567-04-28 at 01 25 38@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/eb8564e7-d3dc-46ac-a8d5-ca8745e2102b)

### Upload my own experience (as JSON)

![CleanShot 2567-04-28 at 01 26 00@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/3e222f35-80da-4b7a-ae17-d517545ed31c)

#### Description

The idea behind is I want to apply RAG on this project. So I think about the scenario that we can use to matching with resume content, Why don't we use some datasource or original resume? This decision as you already know, I was using dataset approach and it's easier for me to use JSON because I have it on my own portfolio site.

Here is current JSON format which acceptable by Backend

Reference: https://github.com/marttp/tpcoder-portfolio/blob/main/src/data/experiences.json

```json
{
  "professional": [
    {
      "company": "company",
      "country": "country",
      "position": "position",
      "startDate": "startDate",
      "endDate": "endDate",
      "highlights": [
        "...list of achievement on this company"
      ]
    }
  ],
  "education": [
    {
      "school": "school",
      "degree": "degree",
      "fieldOfStudy": "fieldOfStudy",
      "startDate": "startDate",
      "endDate": "endDate",
      "courses": [
        "...list of string storing course"
      ]
    }
  ]
}
```

#### Enhancement
- Consider to use PDF file should be general for many use-cases.
- Could be implemented as asynchronous approach. User upload the file => Pass the file to workload which upsert document on vector database.
- For each user upload, right now I read from constant value. However, I believe identify user identity on each upload should be much more usable on the future.

### Upload job description and generate new resume content

![CleanShot 2567-04-30 at 13 47 35@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/f82b17db-a380-4d19-ba9b-42f28c393b88)

#### Description

Find the role you interested, put job title and job description to get your suggestion and score of encouragement to apply your dream role depending on your past experience.

#### Enhancement
- Currently, The result has shown in the group of text. Not the markdown. Meanwhile markdown result is responsed from LLM. I cannot do it on this project because currently, Dioxus don't have official Markdown support on desktop.
- Because my Macbook Pro 2019 13 inches computation power could not process well with LLM project, I think it's better to designed as asynchronous approach as well as upload experience. For this scenarios, Send email about the result or send push notification and store it to inbox should be okay for the UX.
- Better if the final result produce complete version of resume.

## How to run locally

### Prerequisite
- [Git](https://git-scm.com/)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop/) or [Podman](https://podman.io/) or whatever you like e.g. [Colima](https://github.com/abiosoft/colima) to run Qdrant

### Make it run locally
- Clone the repository: `git clone https://github.com/marttp/resumize`

### Run LlamaEdge for your local

Please follow this article about how to run WASM on your locally [Getting Started with Llama-3-8B](https://www.secondstate.io/articles/llama-3-8b/). Below are the steps required for the project only

1. Install WasmEdge via the following command line.
    ```shell
    curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --plugin wasi_nn-ggml
    ```

2. Download the Llama-3-8B model GGUF file.
    ```shell
    curl -LO https://huggingface.co/second-state/Llama-3-8B-Instruct-GGUF/resolve/main/Meta-Llama-3-8B-Instruct-Q5_K_M.gguf
    ```

3. Download an API server app. It is also a cross-platform portable Wasm app that can run on many CPU and GPU devices.
    ```shell
    curl -LO https://github.com/LlamaEdge/LlamaEdge/releases/latest/download/llama-api-server.wasm
    ```

4. Download the chatbot web UI to interact with the model with a chatbot UI.
    ```shell
    curl -LO https://github.com/LlamaEdge/chatbot-ui/releases/latest/download/chatbot-ui.tar.gz
    tar xzf chatbot-ui.tar.gz
    rm chatbot-ui.tar.gz
    ```

5. Start an API server for the model
    ```shell
    wasmedge --dir .:. --nn-preload default:GGML:AUTO:Meta-Llama-3-8B-Instruct-Q5_K_M.gguf \
      llama-api-server.wasm \
      --prompt-template llama-3-chat \
      --ctx-size 4096 \
      --model-name Llama-3-8B
    ```

Feel free to take a look on the mentioned article if you couldn't understand them well. 🙇‍♂️

### Run Qdrant on locally

Depending on your approach, I would suggest to using container technology to speed up on build your playground

#### Standalone container

1. Open terminal and run below command if using docker - reference: https://qdrant.tech/documentation/quick-start/
    ```shell
    docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
    ```

#### Compose file (I used this approach)

1. Navigate to the project directory: `cd actix-rag-qdrant`
2. Open terminal Run relate compose command (This example is using podman)
    ```shell
    podman compose up -d
    ```

### Run Actix Web

1. Navigate to the project directory: `cd actix-rag-qdrant`
2. Run below command to start dev server
     ```shell
     cargo watch -x "run --bin server"
     ```

### Run Dioxus

#### Locally

1. Navigate to the project directory: `cd dx-app`
2. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. Install the tailwind css cli: https://tailwindcss.com/docs/installation
4. Run the following command in the root of the project to start the tailwind CSS compiler:
    ```bash
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
    ```
5. Run the following command in the root of the project to start the Dioxus dev server:
    ```bash
    dx serve --hot-reload --platform desktop
    ```

#### Bundle version (Only for macOS)

1. Follow all step from *Locally* section
2. Stop development mode and run below command instead
    ```bash
    dx bundle --release
    ```
3. If you success, You will have this dmg file.

    ![CleanShot 2567-04-30 at 21 03 36@2x](https://github.com/marttp/resumize/assets/34801905/ec09b53c-d46f-4e72-81dd-14230954cf10)


Apologize for haven't tried on Windows or Linux, I have only 1 laptop. 🙇‍♂️

## License
This project is licensed under the MIT License.

## Contact
If you have any questions, feel free to reach out to me at below contact
- Facebook: [Thanaphoom Babparn](https://www.facebook.com/thanaphoom.mart/)
- FB Page: [TP Coder](https://www.facebook.com/tpcoder)
- LinkedIn: [Thanaphoom Babparn](https://www.linkedin.com/in/thanaphoom-babparn/)
- Website: [TP Coder — Portfolio](https://portfolio.tpcoder.dev/)
