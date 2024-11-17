"use client"
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useState } from "react";

export default function Home() {
  const [inputText, setInputText] = useState()

  const submitInput = (event) => {
    event.preventDefault();
    console.log(inputText)
  }

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <form onSubmit={submitInput}>
          <div className="submit-box">
            <Input onChange={(event) => setInputText(event.target.value)} />
            <Button>submit</Button>
          </div>
        </form>
      </main>
      <style>{`
        .submit-box {
          display: flex;
          gap: 8px;
          flex-direction: row;
        }
      `}</style>
    </div>
  );
}
