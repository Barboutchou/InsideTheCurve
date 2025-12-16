import requests

url = "https://data.norges-bank.no/api/data/SEC/B.NO0010757925.?format=sdmx-json&lastNObservations=60&locale=en"
r = requests.get(url)
data = r.json()

#Affiche toutes les dimensions et codes