"use client"

import * as React from "react"
import { Card, CardHeader } from "./ui/card"

const cardObject = {
    "time": "10:00",
    "place": "千歳空港",
    "activity_name": "飛行機から到着",
    "type": "place"
}

const ResultCard = ({ cardObject }) => {
    return (
        <>
            <div className="result-card">
                <div className="result-card__time">{cardObject.time}</div>
                <Card>
                    <div className="result-card__place">{cardObject.place}</div>
                    <div className="result-card__activity-name">{cardObject.activity_name}</div>
                    <div className="result-card__type">{cardObject.type}</div>
                    </Card>
            </div>
            <style>{`
            .result-card {
                display: flex;
                flex-direction: row;
                align-items: center;
    justify-content: center;
            }
            `}</style>
        </>
    )
}

export { ResultCard }
