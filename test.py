import requests
import re
def beta():
	r = requests.get("https://www.nvaccess.org/nvdaUpdateCheck?autoCheck=1&allowUsageStats=0&versionType=stable")
	match = re.search(r"launcherUrl:\s*(.*)", r.text)
	if not match:
		return "There was an error getting the URL for the latest beta"
	url = match.group(1)
	return url

print(beta())
