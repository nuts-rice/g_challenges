import { writable } from "svelte/store";
import type { TestbooruItem } from "./image_api";

interface SearchUrlParams {
    tags: string[];
    limit: number;
    page: number;
}

export class Search{
     

export const search = (site: string, tags: string[] | string = [], {limit = 20, page = 0}): Promise<TestbooruItem | undefined> => {
    const tagsStore = writable(tags);
    const limitStore = writable(limit);
    const pageStore = writable(page);
    const siteStore = writable(site);
    const tagsGot = tagsStore.subscribe((value) => {
      console.log(value);
    });
    const limitGot = limitStore.subscribe((value) => {
      console.log(value);
    });
    const pageGot = pageStore.subscribe((value) => {
      console.log(value);
    });
    const siteGot = siteStore.subscribe((value) => {
      console.log(value);
    });
    return {
      tags: tagsStore,
      limit: limitStore,
      page: pageStore,
      site: siteStore,
      tagsGot,
      limitGot,
      pageGot,
      siteGot
    };
  }
