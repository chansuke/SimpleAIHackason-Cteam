# call Google Search Class and get the search results

from google_serarch import GoogleSearch
from dotenv import load_dotenv
import os

# .env ファイルをロード
load_dotenv()

if __name__ == '__main__':
    print("Google Search")
    google_search = GoogleSearch(api_key=os.getenv('GOOGLE_API_KEY'))
    response = google_search.get_search_response("沖縄の観光地")