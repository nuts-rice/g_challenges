import {error} from '@sveltejs/kit';

export async function load() {
    try {
        const imgs = import.meta.glob('../assets/*.jpg');

    } catch (err) {
        console.error('Error: ', err);
        throw error(500, 'Failed to load images');
    }
}
