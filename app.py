from flask import Flask, redirect
from lxml import html
import requests
import re

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
	return redirect(url, code=301)

@app.route("/xp")
def xp():
	return redirect("https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe", code=301)

@app.route("/alpha")
def alpha():
	r = requests.get("https://www.nvaccess.org/nvdaUpdateCheck?autoCheck=1&allowUsageStats=0&versionType=snapshot:alpha")
	match = re.search(r"launcherUrl:\s*(.*)", r.text)
	if not match:
		return "There was an error getting the URL for the latest alpha"
	url = match.group(1)
	return redirect(url, code=301)

@app.route("/beta")
def beta():
	r = requests.get("https://www.nvaccess.org/nvdaUpdateCheck?autoCheck=1&allowUsageStats=0&versionType=beta")
	match = re.search(r"launcherUrl:\s*(.*)", r.text)
	if not match:
		return "There was an error getting the URL for the latest beta"
	url = match.group(1)
	return redirect(url, code=301)

@app.errorhandler(404)
def on_not_found(error):
	return "Welcome!\n\nUse /xp to download a Windows XP (2017) release, or no parameter to download a current release."

if __name__ == "__main__":
	app.run(host = "0.0.0.0")
