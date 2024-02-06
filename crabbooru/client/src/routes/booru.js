const Booru = require('booru');

Booru.search('danbooru', ['rating:safe', 'cat'], { limit: 1, random: true })

async function getBooru() {
  const result = await Booru.search('danbooru', ['rating:safe', 'cat'], { limit: 1, random: true })

  return result
}
