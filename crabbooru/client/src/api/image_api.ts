   import { fetch } from "@tauri-apps/api/http"
   import {useState} from "react"
    // import Image from "../components/Image/image.svelte";
    import { invoke } from "@tauri-apps/api/tauri"

    interface UserResponse {
        status_code: number;
        id: number;
        name: string;
    }

    interface Image {
        img: string;
        img_adr: {
            url_list: Array<string>;
        };
    }


   export interface DanbooruItem {
        desc: string;
        img: Image;
    }

    interface DanbooruResponse {
        status_code: number;
        item_list: Array<DanbooruItem>;
    }

    export interface TestbooruItem {
        desc: string;
        img: Image;
    }

    interface SingleTestbooruResponse {
        status_code: number;
        item_list: Array<TestbooruItem>;
    }

    interface MultipleTestbooruResponse {
        status_code: number;
        min_id: number;
        max_id: number;
        has_more: boolean;
        item_list: Array<TestbooruItem>;
    }

    // export function () {
    //     const initial_state = {
    //         status_code: 0,
    //         item_list: [],
    //     };
    // const [test_results, set_test_results] = useState<TestbooruResponse>({status_code: 0, item_list: []});
        export const getDanbooruImage =  async (id: string) : Promise<DanbooruItem | undefined> => {
        try {
        const url = `https://danbooru.donmai.us/posts/${id}.json`;
        const response = await fetch<DanbooruResponse>(url, {
            method: "GET",
        });
        if (response.ok && response.data && response.data.status_code === 200)
            return response.data.item_list[0];
    } catch (error) {
        console.log(error);
    }
};

    export const getTestbooruImage = async (id: string) : Promise<TestbooruItem | undefined> => {
        try {
        const url = `https://testbooru.donmai.us/posts/${id}.json`;
        const response = await fetch<MultipleTestbooruResponse>(url, {
            method: "GET",
        })
        if (response.ok && response.data && response.data.status_code === 200)
            return response.data.item_list[0];
    } catch (error) {
        console.log(error);
    }
    };

    export const getDanbooruImages = async (id: String) => {};

    export const getTestbooruImages = async (id: String) => {};
    export const getTestbooruCallId = async (id: number) => {
        const response = await invoke<SingleTestbooruResponse>('testbooru_call_id', {id: id}).catch((error) => console.error(error))
        return response?.item_list[0];
    }
    export const getDanbooruCall = async (_tags: string[], _page: number, _limit: number) => {
      const response = await invoke<DanbooruResponse>('danbooru_call', {tags: _tags, page: _page,  limit: _limit }).catch((error) => console.error(error))
  return response?.item_list;
    };

   export const getTestbooruCall = async (_tags: string[], _page: number, _limit: number) => {
   
    // try {
      const response = await invoke<MultipleTestbooruResponse>('testbooru_call', {tags: _tags, page: _page,  limit: _limit }).catch((error) => console.error(error))
  return response?.item_list;
        // if (response. && response.data && r<ScrollWheelUp>esponse.data.status_code === 200)

      // console.log(response);
        // return response;
    };
    
    



