import {createEventDispatcher} from 'svelte';
const dispatch = createEventDispatcher();
dispatch('search', {tags: 'tags'});



