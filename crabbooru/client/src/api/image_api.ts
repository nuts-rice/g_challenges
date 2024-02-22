import { fetch } from "@tauri-apps/api/http";
// import { useState } from "react";
// import Image from "../components/Image/image.svelte";
import { invoke } from "@tauri-apps/api/tauri";
import Post from "./post_api";
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

export interface SingleDanbooruResponse {
  status_code: number;
  item_list: Array<DanbooruItem>;
}
export interface MultipleDanbooruResponse {
  status_code: number;
  min_id: number;
  max_id: number;
  has_more: boolean;
  item_list: Array<TestbooruItem>;
}

export interface TestbooruItem {
  desc: string;
  img: Image;
}

export interface SingleTestbooruResponse {
  status_code: number;
  item_list: Array<TestbooruItem>;
}

export interface MultipleTestbooruResponse {
  status_code: number;
  min_id: number;
  max_id: number;
  has_more: boolean;
  item_list: Array<TestbooruItem>;
}


export const BooruSite  ={
  Testbooru: 0,
  Safebooru : 1,
  Danbooru: 2,
}

// export function () {
//     const initial_state = {
//         status_code: 0,
//         item_list: [],
//     };
// const [test_results, set_test_results] = useState<TestbooruResponse>({status_code: 0, item_list: []});
export const getDanbooruImage = async (
  id: string
): Promise<DanbooruItem | undefined> => {
  try {
    const url = `https://danbooru.donmai.us/posts/${id}.json`;
    const response = await fetch<SingleDanbooruResponse>(url, {
      method: "GET",
    });
    if (response.ok && response.data && response.data.status_code === 200)
      return response.data.item_list[0];
  } catch (error) {
    console.log(error);
  }
};

export const getTestbooruImage = async (
  id: string
): Promise<TestbooruItem | undefined> => {
  try {
    const url = `https://testbooru.donmai.us/posts/${id}.json`;
    const response = await fetch<MultipleTestbooruResponse>(url, {
      method: "GET",
    });
    if (response.ok && response.data && response.data.status_code === 200)
      return response.data.item_list[0];
  } catch (error) {
    console.log(error);
  }
};

export const getDanbooruImages = async (id: String) => {};

export const getTestbooruImages = async (id: String) => {};

export const getTestbooruCallId = async (
  id: number
) => {
try {
  const response = await invoke<SingleTestbooruResponse>("testbooru_call_id", {
    id: id,
  });
  if (response.status_code == 200)  
  console.log("get_testbooru_call_id: " + response)
  const img_url = await invoke<string>("testbooru_post_img", {
    post: response,
    });
    console.log("get_testbooru_call_id: img_url :" + img_url);
  return img_url
  } catch (error) {
  console.log(error);
   } 
   }

// export const parseTestbooruImageUrl = (imageUrl: string): string | null => {}

  


export const viewTestbooruImage = async (id: number) => {}

export const getDanbooruCall = async (
  _tags: string[],
  _page: number,
  _limit: number
) : Promise<MultipleDanbooruResponse | undefined> => {
  try {
  const response = await invoke<MultipleDanbooruResponse>("danbooru_call", {
    tags: _tags,
    page: _page,
    limit: _limit,
  })
  if (response.status_code == 200)
  console.log("get_danbooru_call: " + response);
  return response;
  } catch (error) {
  console.log(error);
  }
};

export const getTestbooruCall = async (
  _tags: string[],
  _page: number,
  _limit: number
) => {
  try {
  const response = await invoke<MultipleTestbooruResponse>("testbooru_call", {
    tags: _tags,
    page: _page,
    limit: _limit,
  })
  if (response.status_code == 200)
  console.log("get_testbooru_call: " + response);
  return response;
  } catch (error) {
  console.log(error);
  }
  // if (response. && response.data && r<ScrollWheelUp>esponse.data.status_code === 200)

  // console.log(response);
  // return response;
};

export const getTestImg = async (_response: SingleTestbooruResponse): Promise<string| undefined> => {
  try {
    const url = _response.item_list[0].img.img_adr.url_list[0];
     
    console.log("get_test_img: url :" + url);
    return url;
  } catch (error) {
  console.log(error);
    }
}

export const getSingleTestbooruImg = async (_post: TestbooruItem) => {
  try {
    const img_url = await invoke<string>("testbooru_post_img", {
    post: _post,
  })
  // if (post.img.img_adr.url_list[0] == img_url)
  console.log("get_single_testbooru_img: url :" + img_url);
  return img_url;
  } catch (error) {
  console.log(error);
  }

};

export default class BooruResult extends Array<Post> {
public booru_id: number
public page: number
public readonly tags: string[]
public readonly posts: Post[] 

constructor(booru_id: number, page: number, tags: string[], posts: Post[]) {
  super(posts.length)
  for (let i = 0; i < posts.length; i++) {
    this[i] = posts[i]
  }
  this.booru_id = booru_id
  this.page = page
  this.tags = tags
  this.posts = posts
}
public booru_call = async (_tags: string[],
  _page: number,
  _limit: number
) => {
  try {
  const response = await invoke<BooruResult>("booru_call_test", {booru: this.booru_id, tags: _tags, page: _page, limit: _limit})
      // if (response.status_code == 200)
      console.log("booru_call: response: " + response);
      return response;
    }catch (error) {
  console.log(error);
  }
}

public parseBooruCall = async (response: any) => {
      

}

}
