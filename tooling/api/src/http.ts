import {invoke} from "./core";

export interface RequestOptions{
    body?: BodyInit | null;
    headers?: HeadersInit;
    method?: string;
}

const convertToArrayBuffer = async (bodyInit: BodyInit | undefined | null): Promise<number[] | null> => {
    if(!bodyInit){
        return null;
    }
    const response = new Response(bodyInit);
    return Array.from(await response.bytes())
}

export namespace http{
    export const fetch = async (
        request: string | URL,
        options?: RequestOptions,
    ): Promise<Response> => {
        if(options?.body){
            // @ts-ignore
            options.body = await convertToArrayBuffer(options.body)
        }
        const output =  await invoke<{
            body: number[];
            headers: Record<string, string>;
            status: number;
            statusText: string;
        }>("FLURX|http::fetch", {
            url: request.toString(),
            ...options,
        });
        return new Response(new Uint8Array(output.body), {
            headers: output.headers,
            status: output.status,
            statusText: output.statusText,
        })
    }
}