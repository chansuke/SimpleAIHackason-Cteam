"use client";

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { useState, useEffect } from "react";
import { useSearchParams } from 'next/navigation';
import { ResultCard } from "@/components/resultCard";

export default function Result() {
    const searchParams = useSearchParams();
    const country = searchParams.get("country");
    const city = searchParams.get("city");
    const query = searchParams.get("query");

    const [resultList, setResultList] = useState([]); // 修正: 状態管理追加
    const [loading, setLoading] = useState(true); // ローディング状態
    const [error, setError] = useState(null); // エラーメッセージ

    useEffect(() => {
        (async () => {
            try {
                setLoading(true); // ローディング開始
                console.log({
                    country,
                    city,
                    query
                })
                const response = await fetch("http://localhost:8000/chat", {
                    method: "POST",
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        country,
                        city,
                        query
                    }),
                });

                if (!response.ok) {
                    throw new Error(`API Error: ${response.status}`);
                }
                setResultList(await response.json())

            } catch (err) {
                setError(err.message); // エラーを状態に設定
            } finally {
                setLoading(false); // ローディング終了
            }
        })();
    }, [country, city, query]);

    return (
        <div className="result-container">
            {loading && <p>Loading...</p>} {/* ローディング表示 */}
            {error && <p className="error">Error: {error}</p>} {/* エラー表示 */}

            {!loading && !error && resultList.map((elm, index) => (
                <ResultCard cardObject={elm}/>
            ))}
            <style>{`
                .result-container {
                    display: flex;
                    flex-direction: column;
                    gap: 16px;
                }
                .result-element {
                    border: 1px solid #ddd;
                    border-radius: 8px;
                    padding: 16px;
                    background-color: #fff;
                }
                .error {
                    color: red;
                }
            `}</style>
        </div>
    );
}
