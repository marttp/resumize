# Resumize (เรซูมั้ย?)

## Overview
This application helps generate resume content based on your experience and the job description of your interest.

![CleanShot 2567-04-28 at 01 25 21@2x](https://github.com/marttp/rag-resume-generate/assets/34801905/b04e820e-68a5-4c15-b93c-c43aad035fe6)

## Article
- EN
- TH

## Tech Stack
- [Dioxus](https://dioxuslabs.com/) 0.5.1
- [Actix Web](https://actix.rs/) 4.5.1
- [Qdrant](https://qdrant.tech/) 1.9.0
- [LlamaEdge](https://github.com/LlamaEdge/LlamaEdge) with [Meta Llama 3](https://llama.meta.com/llama3/)

## Features
- **Generate Resume Content**: Input your dream job description and get a customized resume content. Please note that the result is not perfect.
- **Upload Experience**: Upload your past experiences in JSON format to be used for generating the resume content.

## Demo
[Link to your demo video]

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

TBC

1. Navigate to the project directory: `cd resume-generator`
2. Build the project: `cargo build --release`

### Run Qdrant

TBC

1. Navigate to the project directory: `cd resume-generator`
2. Build the project: `cargo build --release`

### Run Actix Web

TBC

1. Navigate to the project directory: `cd resume-generator`
2. Build the project: `cargo build --release`

### Run Dioxus

TBC

#### Locally

1. Navigate to the project directory: `cd resume-generator`
2. Build the project: `cargo build --release`

#### Bundle version

1. Navigate to the project directory: `cd resume-generator`
2. Build the project: `cargo build --release`

## License
This project is licensed under the MIT License.

## Contact
If you have any questions, feel free to reach out to me at below contact
- Facebook: [Thanaphoom Babparn](https://www.facebook.com/thanaphoom.mart/)
- FB Page: [TP Coder](https://www.facebook.com/tpcoder)
- LinkedIn: [Thanaphoom Babparn](https://www.linkedin.com/in/thanaphoom-babparn/)
- Website: [TP Coder — Portfolio](https://portfolio.tpcoder.dev/)
