import UploadImage, { UploadProps } from "./UploadImage";

const Upload = (props: UploadProps) => {

  return (
    <UploadImage
      ipfs={props.ipfs}
      connection={props.connection}
      wallet={props.wallet}
      program={props.program}
    />
  );
};

export default Upload;
