# nvda_zip
This is a very simple flask app that runs at https://nvda.zip, providing a simple API to download the latest version of the NVDA screen reader.

## Endpoints:
* /: download the latest stable NVDA version.
* /stable.json: get a JSON response containing NVDA's current stable version number.
* /xp: download the last NVDA version to run on Windows XP.
* /xp.json: get a JSON response containing the last version of NVDA that ran on Windows XP.
* /alpha: download the latest NVDA snapshot version.
* /alpha.json: get a JSON response containing the latest alpha version number.
* /beta: download the latest NVDA beta version.
* /beta.json: get a JSON response containing the current NVDA beta version number.

## Todo:
* Add an endpoint for downloading the last NVDA version to run on Windows 7 and Vista.
