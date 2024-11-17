from google_serarch import GoogleSearch
from search_url_details import SearchUrlDetails
import openai
import json
from dotenv import load_dotenv
import os

# .env ファイルをロード
load_dotenv()

system_role = "あなたは優秀な観光ガイドです。主要な観光地はもちろん穴場な観光地まで知っています。"
example_input = """
次の前提を踏まえて、旅行のタイムラインを作成してください。

クエリ: 札幌の観光地の情報を教えてください
前提: 北海道
"""
example_output = """
以下の形式で出力してください:
{
  "time": "10:00"
  "place": "千歳空港"
  "activity_name": "飛行機から到着"
  "type": "place"
},
{
  "time": "12:00"
  "place": "向坂食堂"
  "activity_name": "スープカレーを食べる"
  "type": "food"
},
{
  "time": "15:00"
  "place": "小岩井農場"
  "activity_name": "乗馬体験をする"
  "type": "activity"
},
"""

question = input("")
# question = "札幌の観光情報を元に案内を作ってください"

google_search = GoogleSearch(api_key=os.getenv('GOOGLE_API_KEY'))
search_links = google_search.get_search_response(question)

teach_data = ""
for link in search_links:
    res = SearchUrlDetails.get_search_url_details(link)
    teach_data += res

search_result = teach_data[:5000]

real_input = """次の前提を踏まえて、旅行のタイムラインを作成してください。

クエリ: {}
前提: {}
""".format(search_result, question)

response = openai.chat.completions.create(
    model="gpt-4",
    temperature=0,
    messages=[
            {"role":"system","content":system_role},
            {"role":"user","content":example_input},
            {"role":"assistant","content":example_output},
            {"role":"user","content":real_input}
        ]
    )

print(response.choices[0].message.content)