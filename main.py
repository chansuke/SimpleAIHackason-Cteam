from flask import Flask, jsonify, request
from google_serarch import GoogleSearch
from search_url_details import SearchUrlDetails
from dotenv import load_dotenv
import os

# .env ファイルをロード
load_dotenv()

app = Flask(__name__)

@app.route('/')
def home():
    return "Welcome to the Google Search API"

@app.route('/search')
def search():
    keyword = request.args.get('keyword', default='沖縄の観光地', type=str)
    google_search = GoogleSearch(api_key=os.getenv('GOOGLE_API_KEY'))
    response = google_search.get_search_response(keyword)
    return jsonify(response)

if __name__ == '__main__':
    app.run(debug=True)