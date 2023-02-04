# rust-chromium-azure-fucntions-crawler-poc
A PoC to verify the performance of a crawler/scraper built in Rust and a Headless Chrome (Chromium) to pre-render and scrape websites - all of this hosted serverless on Azure Functions.

Note:
This is just a PoC and far from being production-ready. I just wanted to quickly verify the idea of using Azure Functions and Rust to build a simple crawler to return the HTML DOM of websites. I hope it is useful for someone. PRs/Forks are welcome.

I explained the whole idea in this [video](https://www.youtube.com/watch?v=f6t7XnS3Rbs) (Please like and subscribe if is useful :) )

How to run:
- Follow the steps on the [Azure Functions Deployment Guide for Rust](https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Cmacos)

Completed:
- Deployment on Azure Functions 
- HTML DOM of target website is being returned by the function
- Streamlined error handling
- Wait for DOM elements using CSS selectors (useful to scrape SPAs)

Open:
- Wait fixed time amount before scraping the DOM
- Support proxy configuration in the GET request (e.g. GET /..&own_proxy=https%3A%2F%2Fjohndoe%3Apassword%40my_proxy.com%3A1234 )

Learnings:
- Using Rust instead of JavaScript (Node.js) on Azure Functions reduces cold-start times by nearly 10x. This maybe reasoned by the very small build size of  11MB (not even using release target to build the executable).
- Rust is still not first-class citizen of Azure Functions. Cloudflare Workers is [supporting it officially](https://blog.cloudflare.com/workers-rust-sdk/) 
- Bigger websites use fingerprinting techniques and block datacenter IP blocks to prevent being crawled.
