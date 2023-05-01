import { AxiosResponse } from 'axios';
import { useRef, useState } from 'react';

const DEFAULT_EXPORT_FILE_NAME: string = 'constants';

export interface ExportFileProps {
  readonly apiDefinition: () => Promise<AxiosResponse<Blob | string>>;
  readonly preExport: () => void;
  readonly postExport: () => void;
  readonly onError: () => void;
}

export interface ExportedFileInfo {
  readonly exportHandler: () => Promise<void>;
  readonly ref: React.MutableRefObject<HTMLAnchorElement | null>;
  readonly name: string | undefined;
  readonly url: string | undefined;
}

export const useExportFile = ({
  apiDefinition,
  preExport,
  postExport,
  onError,
}: ExportFileProps): ExportedFileInfo => {
  const ref = useRef<HTMLAnchorElement | null>(null);
  const [url, setFileUrl] = useState<string>();
  const [name, setFileName] = useState<string>();

  const exportHandler = async () => {
    try {
      preExport();
      // console.log("API Definition: ", apiDefinition);
      const response = await apiDefinition();
      // console.log("Response: ", response)
      // console.log("CHECK: ", URL);
      const objectURL = URL.createObjectURL(
        new Blob([response.data], { type: response.headers['content-type'] }),
      );
      // console.log("Object URL", objectURL);
      setFileUrl(objectURL);
      // console.log(response.headers['content-disposition'].match(/filename="(.+)"/));
      const fileName =
        response.headers['content-disposition'].match(/filename="(.+)"/)[1] ??
        DEFAULT_EXPORT_FILE_NAME;
      // console.log(fileName);
      setFileName(fileName);
      console.log(ref);
      ref.current?.click();
      postExport();
      if (url) URL.revokeObjectURL(url);
    } catch (error) {
      console.log(error)
      onError();
    }
  };

  return { exportHandler, ref, url, name };
};
