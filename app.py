from flask import Flask, redirect
from lxml import html
import requests

app = Flask(__name__)

@app.route("/")
def index():
	r = requests.get("https://www.nvaccess.org/download", headers={
		"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36"
	})
	r.raise_for_status()
	tree = html.fromstring(r.text)
	current_version = tree.xpath("//div[@class='download-version__details section-title-v2']//h2")[0].text.replace("NVDA version ", "").strip()
	url = f"https://www.nvaccess.org/download/nvda/releases/{current_version}/nvda_{current_version}.exe"
	return redirect(url, code=302)

@app.route("/xp")
def index_xp():
	return redirect("https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe", code=302)

@app.errorhandler(404)
def on_not_found(error):
	return "Welcome!\n\nUse /xp to download a Windows XP (2017) release, or no parameter to download a current release."

if __name__ == "__main__":
	app.run()
