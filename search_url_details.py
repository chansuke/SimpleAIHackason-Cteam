#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import requests
from bs4 import BeautifulSoup

class SearchUrlDetails:
    def get_search_url_details(url):
        response = requests.get(url)
        if response.status_code == 200:
            soup = BeautifulSoup(response.text, 'html.parser')
            body = soup.body
            if body:
                return body.get_text(strip=True)
            else:
                return "No body element found"
        else:
            return f"Failed to retrieve data: {response.status_code}"
