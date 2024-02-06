<script lang="ts">
    import { fetch } from "@tauri-apps/api/http";
    import Image from "../components/Image/image.svelte";

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


    interface DanbooruItem {
        desc: string;
        img: Image;
    }

    interface DanbooruResponse {
        status_code: number;
        item_list: Array<DanbooruItem>;
    }

    interface TestbooruItem {
        desc: string;
        img: Image;
    }

    interface TestbooruResponse {
        status_code: number;
        item_list: Array<TestbooruItem>;
    }

    export const getDanbooruImage =  async (id: String) => {
        const url = `https://danbooru.donmai.us/posts/${id}.json`;
        const response = await fetch<DanbooruResponse>(url, {
            method: "GET",
            query: { id },
        }).catch((error) => console.error(error));
        if (response && response.data && response.data.status_code === 0)
            return response.data.item_list[0];
    };

    export const getTestbooruImage = async (id: String) => {
        const url = `https://testbooru.com/posts/${id}.json`;
        const response = await fetch<TestbooruResponse>(url, {
            method: "GET",
            query: { id },
        }).catch((error) => console.error(error));
        if (response && response.data && response.data.status_code === 0)
            return response.data.item_list[0];
    };

    export const getDanbooruImages = async (id: String) => {};

    export const getTestbooruImages = async (id: String) => {};

</script>
