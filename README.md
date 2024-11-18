# simpleSearch

**simpleSearch** is a Rust-based command-line tool that enables users to perform web searches and receive AI-generated summaries of search results. The application is designed to be fast, efficient, and user-friendly, combining the power of web scraping with AI language models to provide concise information right in your terminal.

## Features

- **Web Search**: Quickly search the web using a chosen search engine.
- **AI Summarization**: Generate concise AI summaries of the top search results.
- **Customization**: Choose search engine preferences(future) and set the number of results to summarize.

## Installation

### Prerequisites

- Rust and Cargo installed. You can follow the official installation instructions [here](https://www.rust-lang.org/tools/install).
- API keys for any web search services and AI summarization services used (e.g., Bing Search API, OpenAI API).

## Usage
    ### Usage

    To use **simpleSearch**, run the following command in your terminal:

    ```sh
    simpleSearch [FLAGS] [OPTIONS]
    ```

    #### Flags and Options

    - `-q`, `--query` : Specify the search query(required).
    - `-c`, `--chat` : Enable chatbot summary of the search results(default: false).
    - `-n`, `--number_of_results` : Set the number of search results to summarize(default: 5).
    - `-s`, `--search` : Search Engine(default: Google) Available: Google

    #### Examples

    Search for a query and get a summary:
    ```sh
    simpleSearch -q "Rust programming language" -c
    ```

    Search for a query with a specified number of results:
    ```sh
    simpleSearch -q "Rust programming language" -n 5
    ```