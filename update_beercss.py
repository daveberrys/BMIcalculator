import json
import urllib.request

# variables
beercss_npm_json = "https://registry.npmjs.org/beercss/latest"
mdc_npm_json = "https://registry.npmjs.org/material-dynamic-colors/latest"
npm_cdn = "https://cdn.jsdelivr.net/npm"

print("get latest tag")
with urllib.request.urlopen(beercss_npm_json) as r:
    beercss_latest_version = json.load(r)
    print("  got beercss latest version")
with urllib.request.urlopen(mdc_npm_json) as r:
    mdc_latest_version = json.load(r)
    print("  got mdc latest version")

print("get beercss code")
with urllib.request.urlopen(f"{npm_cdn}/{beercss_latest_version['_id']}/dist/cdn/beer.min.css") as r:
    temp = r.read()
    beercss_min_css = temp.decode('utf-8')
    print("  got beercss code")
with urllib.request.urlopen(f"{npm_cdn}/{beercss_latest_version['_id']}/dist/cdn/beer.min.js") as r:
    temp = r.read()
    beercss_min_js = temp.decode('utf-8')
    print("  got beerjs code")
with urllib.request.urlopen(f"{npm_cdn}/{mdc_latest_version['_id']}/dist/cdn/material-dynamic-colors.min.js") as r:
    temp = r.read()
    mdc_min_js = temp.decode('utf-8')
    print("  got mdc code")

print("write it all")
with open("assets/beercss/beercss.min.css", "w") as f:
    f.write(beercss_min_css)
    print("  wrote beercss code")
with open("assets/beercss/beer.min.js", "w") as f:
    f.write(beercss_min_js)
    print("  wrote beerjs code")
with open("assets/beercss/material-dynamic-colors.min.js", "w") as f:
    f.write(mdc_min_js)
    print("  wrote mdc code")
