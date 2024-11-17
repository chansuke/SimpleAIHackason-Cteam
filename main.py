# call Google Search Class and get the search results

from google_serarch import GoogleSearch
from search_url_details import SearchUrlDetails
from dotenv import load_dotenv
import os

# .env ファイルをロード
load_dotenv()

if __name__ == '__main__':
    # print("Google Search")
    # google_search = GoogleSearch(api_key=os.getenv('GOOGLE_API_KEY'))
    # response = google_search.get_search_response("沖縄の観光地")
    # return [link, link, link, ...]
    # print(response)

    url = "https://www.google.co.jp/travel/hotels/entity/ChkQyeyJ9rS9scZ4Gg0vZy8xMWtqNW5qZ2NnEAI"
    response = SearchUrlDetails.get_search_url_details(url)
    print(response)
