from flask import Flask, redirect, jsonify, render_template
import requests
import re

app = Flask(__name__)

def get_url(type):
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

@app.route("/stable.json")
def stable_json():
	json = {}
	url = get_url("stable")
	if url:
		json["url"] = url
	return jsonify(json)

@app.route("/xp")
def xp():
	return redirect("https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe", code=301)

@app.route("/xp.json")
def xp_json():
	json = {}
	json["url"] = "https://www.nvaccess.org/download/nvda/releases/2017.3/nvda_2017.3.exe"
	return jsonify(json)

@app.route("/alpha")
def alpha():
	url = get_url("snapshot:alpha")
	if not url:
		return "There was an error getting the latest NVDA alpha version"
	return redirect(url, code=301)

@app.route("/alpha.json")
def alpha_json():
	json = {}
	url = get_url("snapshot:alpha")
	if url:
		json["url"] = url
	return jsonify(json)

@app.route("/beta")
def beta():
	url = get_url("beta")
	if not url:
		return "There was an error getting the latest NVDA beta"
	return redirect(url, code=301)

@app.route("/beta.json")
def beta_json():
	json = {}
	url = get_url("beta")
	if url:
		json["url"] = url
	return jsonify(json)

@app.errorhandler(404)
def on_not_found(error):
	return render_template("404.html")

if __name__ == "__main__":
	app.run(host = "0.0.0.0")
