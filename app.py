from flask import Flask, redirect
from lxml import html
import requests
import re

app = Flask(__name__)

def get_url(type):
	print(type)
	url = ""
	r = requests.get(f"https://www.nvaccess.org/nvdaUpdateCheck?versionType={type}")
	r.raise_for_status()
	match type:
		case "snapshot:alpha":
			match = re.search(r"launcherUrl:\s*(.*)", r.text)
			if not match:
				return None
			url = match.group(1)
		case "beta" | "stable":
			print(r.text)
			match = re.search(r"version:\s*(.*)", r.text)
			if not match:
				return None
			version = match.group(1).strip()
			url = f"https://www.nvaccess.org/download/nvda/releases/{version}/nvda_{version}.exe"
	return url

@app.route("/")
def index():
	url = get_url("stable")
	if not url:
		return "There was an error getting the latest stable NVDA version"
	return redirect(url, code=301)

@app.route("/xp")
def xp():
	return redirect("https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe", code=301)

@app.route("/alpha")
def alpha():
	url = get_url("snapshot:alpha")
	if not url:
		return "There was an error getting the latest NVDA alpha version"
	return redirect(url, code=301)

@app.route("/beta")
def beta():
	url = get_url("beta")
	if not url:
		return "There was an error getting the latest NVDA beta"
	return redirect(url, code=301)

@app.errorhandler(404)
def on_not_found(error):
	return "Welcome!\n\nUse /xp to download a Windows XP (2017) release, or no parameter to download a current release."

if __name__ == "__main__":
	app.run(host = "0.0.0.0")
