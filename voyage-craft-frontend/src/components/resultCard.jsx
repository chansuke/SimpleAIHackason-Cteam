"use client"

import * as React from "react"
import { Card, CardHeader, CardContent, CardFooter } from "./ui/card"
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "./ui/input"
import { Button } from "./ui/button"
import { Copy } from "lucide-react"

const cardObject = {
    "time": "10:00",
    "place": "千歳空港",
    "activity_name": "飛行機から到着",
    "type": "place"
}

const openModal = () => { }


const ResultCard = ({ cardObject }) => {
    return (
        <>
            <Dialog>
                <DialogTrigger asChild>
                    <div className="result-card">
                        <div className="result-card__time">{cardObject.time}</div>
                        <Card>
                            <CardHeader>
                                <div className="result-card__place">{cardObject.place}</div>
                                <div className="result-card__activity-name">{cardObject.activity_name}</div>
                            </CardHeader>
                            <CardFooter>
                                <div className="result-card__type">{cardObject.type}</div>
                            </CardFooter>
                        </Card>
                    </div>
                </DialogTrigger>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>{cardObject.place}</DialogTitle>
                        <DialogDescription>
                            {cardObject.activity_name}
                        </DialogDescription>
                    </DialogHeader>
                    <Card>
                        <CardHeader>
                            概要
                        </CardHeader>
                        <CardContent>
                            {cardObject.description}
                        </CardContent>
                    </Card>
                    <DialogFooter className="sm:justify-start">
                        <DialogClose asChild>
                            <Button type="button" variant="secondary">
                                Close
                            </Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
            <style>{`
            .result-card {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-evenly;

            }
            `}</style>
        </>
    )
}

export { ResultCard }
