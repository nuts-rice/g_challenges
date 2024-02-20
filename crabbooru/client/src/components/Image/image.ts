import {getDanbooruImage} from '../../api/image_api';    
import {getTestbooruImage} from '../../api/image_api';

interface BooruImageProps extends HTMLImageElement {
    src: string;
    alt: string;
    width: number;
    height: number;
}

export function BooruImage({
   src,  
   alt, 
   width,
   height,  
   ...props
}: BooruImageProps) {
   const handleClick = (url: string) => {
        const filename = url.split("/").pop();    
        console.log("handleClick: " + filename);
   } 
//    return (
//    <Image src=src alt=alt width=width height=height/>
//     <div>
//         <Button on:click={() => handleClick(src)}> 
//             Download Image 
//            </Button>
   
//    )
// }  
   

</script>

