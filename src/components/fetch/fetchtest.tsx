import { createSignal } from "solid-js";

export default function FetchTest(){

    const url = 'http://127.0.0.1:8080/json'

    const data = {
        name: 'john',
        age: 45,
    };

    async function rustback(){
        const response = await fetch(url,{
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': "http://127.0.0.1:8080/json"
            },
            body: JSON.stringify(data)
        })

     

        console.log(response.json())
    }
    
    return(
        <>
        <div >
            <button onclick={rustback}>click here for hello</button>
        </div>
        </>
    )
}