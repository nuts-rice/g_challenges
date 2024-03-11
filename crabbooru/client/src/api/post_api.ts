

interface PostDetails {}

function parseImageUrl(imageUrl: string): string | null{
  return imageUrl;
}

function getTags(): string[] {
  return [];
}

export default class Post {
  public boorus: string[];
  public page: number;
  public postDetails: PostDetails;
  public id: string;
  public readonly tags: string[];
  constructor(id: string, boorus: string[], page: number, postDetails: PostDetails) {
    this.id = id;
    this.boorus = boorus;
    this.page = page;
    this.postDetails = postDetails;
    this.tags = getTags();
  }
}
