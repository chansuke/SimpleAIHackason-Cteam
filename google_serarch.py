#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import datetime
import json

from time import sleep
from googleapiclient.discovery import build

class GoogleSearch:
    DATA_DIR = 'data'
    CUSTOM_SEARCH_ENGINE_ID = "41ebaeed608d14f83"

    def __init__(self, api_key):
        self.api_key = api_key

    def make_dir(self, path):
        if not os.path.isdir(path):
            os.mkdir(path)

    def get_search_response(self, keyword):
        today = datetime.datetime.today().strftime("%Y%m%d")
        timestamp = datetime.datetime.today().strftime("%Y/%m/%d %H:%M:%S")

        self.make_dir(self.DATA_DIR)

        service = build("customsearch", "v1", developerKey=self.api_key)

        page_limit = 10
        start_index = 1
        response = []
        for n_page in range(0, page_limit):
            try:
                sleep(1)
                response.append(service.cse().list(
                    q=keyword,
                    cx=self.CUSTOM_SEARCH_ENGINE_ID,
                    lr='lang_ja',
                    num=10,
                    start=start_index
                ).execute())
                start_index = response[n_page].get("queries").get("nextPage")[0].get("startIndex")
            except Exception as e:
                print(e)
                break

        # レスポンスをjson形式で保存
        save_response_dir = os.path.join(self.DATA_DIR, 'response')
        self.make_dir(save_response_dir)
        out = {'snapshot_ymd': today, 'snapshot_timestamp': timestamp, 'response': []}
        out['response'] = response
        jsonstr = json.dumps(out, ensure_ascii=False)
        with open(os.path.join(save_response_dir, 'response_' + today + '.json'), mode='w') as response_file:
            response_file.write(jsonstr)
        return response

# 使用例
# google_search = GoogleSearch(api_key=GOOGLE_API_KEY)
# response = google_search.get_search_response("example keyword")
