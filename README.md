# Mozilla Readability 

Mozilla Readability attempts to extract the relevant content of an HTML document returning a parsed and cleaned version. 

## Example

```
import * as readability from './pkg/readability_wasm.js'
const instance = new readability.Readability();
const testString = `
    <html>
        <head>
            <link rel="stylesheet">
        </head>
        <body>
            <nav>
                <ul>
                    <li>
                        Home
                    </li>
                    <li>
                        Not Home
                    </li>
                </ul>
            </nav>  
            <div>
                <article>
                    <h1>I am a test</h1>
                    <p>lorem lorem</p>
                    <p>ipsumity ipsum</p>
                </article>
            </div>
        </body>

    </html>

`;

const results = instance.parse();
console.log({
    title: "I am a test",
    byline: null,
    excerpt: "lorem lorem ipsum ipsum",
    site_name: "example.com",
    article_html: `
        <article>
            <h1>I am a test</h1>
            <p>lorem lorem</p>
            <p>ipsumity ipsum</p>
        </article>
    `,
});

```
