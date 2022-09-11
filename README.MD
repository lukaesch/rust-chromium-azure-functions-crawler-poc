# rust-chromium-azure-fucntions-crawler-poc
A PoC to verify the performance of a crawler built with Rust and a Headless Chrome (Chromium) to pre-render website hosted on Azure Functions.

Note:
This is just a PoC and far from being production-ready. I just wanted to quickly verify the idea of using Azure Functions and Rust to build a simple crawler to return the HTML DOM of websites. I hope it is useful for someone. PRs/Forks are welcome.

How to run:
- Follow the steps on the [Azure Functions Deployment Guide for Rust](https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Cmacos)

Completed:
- Deployment on Azure Functions 
- HTML DOM of target website is being returned by the function

Open:
- Streamline error handling
- Figure out how we get the DOM output of SPAs

Learnings:
- Using Rust instead of JavaScript (Node.js) on Azure Functions reduces cold-start times by nearly 10x. This maybe reasoned by the very small build size of  11MB (not even using release target).
- Rust is still not first-class citizens of Azure Functions. Cloudflare Workers is [supporting it officially](https://blog.cloudflare.com/workers-rust-sdk/) 
- Node.js ecosystem is more mature: One could use Pupeteeer/Playwright to reach the goal.
- Bigger websites use fingerprinting techniques and block datacenter IP blocks to prevent being crawled. 