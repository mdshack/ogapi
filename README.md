# ogapi

Rust-based API to easily fetch Open Graph tags from any URL.

This project is very much a _toy-project_, not hardened for production use. Please use at your own caution.

## Usage

### Docker


1. Start the server using: `docker run -it -p 3000:3000 mdshack/ogapi:latest`

```sh
curl http://127.0.0.1:3000/?url=https://www.theverge.com/editorial/626356/galaxy-z-flip-folding-phone-didnt-last-two-years-battery | jq
```

### From Source

1. Pull down the repository and navigate to the folder
2. Start the server using `cargo run` 
3. Make a request to the server, adding a `url` query param (the URL you wish to get OpenGraph for)

```sh
curl http://0.0.0.0:3000/?url=https://www.theverge.com/editorial/626356/galaxy-z-flip-folding-phone-didnt-last-two-years-battery | jq
```

### Example Response

```json
{
  "og:title": "How Samsung’s Galaxy Z Flip failed me without actually breaking",
  "og:type": "article",
  "og:url": "https://www.theverge.com/editorial/626356/galaxy-z-flip-folding-phone-didnt-last-two-years-battery",
  "og:image": "https://platform.theverge.com/wp-content/uploads/sites/2/2025/03/samsung-z-flip-5-sean-hollister-verge-331A1104.jpg?quality=90&strip=all&crop=9.2%2C20.234211349853%2C77.6%2C60.942536846537&w=1200",
  "og:site_name": "The Verge",
  "og:description": "The battery — among other things.",
  "fb:page_id": "",
  "application-name": "",
  "og:email": "",
  "og:phone_number": "",
  "og:fax_number": "",
  "og:latitude": "",
  "og:longitude": "",
  "og:street-address": "",
  "og:locality": "",
  "og:region": "",
  "og:postal-code": "",
  "og:country-name": "",
  "fb:admins": "",
  "og:points": "",
  "og:video": "",
  "og:video:height": "",
  "og:video:width": "",
  "og:video:type": "",
  "og:audio": "",
  "og:audio:title": "",
  "og:audio:artist": "",
  "og:audio:album": "",
  "og:audio:type": ""
}
```