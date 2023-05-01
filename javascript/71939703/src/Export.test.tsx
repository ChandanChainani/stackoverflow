// import React from 'react';
// import ReactDOM from 'react-dom/client';
import { act } from 'react-dom/test-utils';
import { AxiosResponse } from 'axios';
// import {render} from '@testing-library/react'
// import '@testing-library/jest-dom'
import { render } from '@testing-library/react';
import { renderHook } from '@testing-library/react-hooks';
import { useExportFile } from './Export';

describe('useExportFile', () => {
  const URL = "some-url";
  const blob: Blob = new Blob(['']);
  const FILE_NAME = "some-file"

  const axiosResponse: AxiosResponse = {
    data: blob,
    status: 200,
    statusText: "OK",
    config: {},
    headers: {
      "content-disposition": `filename="${FILE_NAME}"`,
      "content-type": "text"
    }
  };

  global.URL.createObjectURL = () => URL;
  global.URL.revokeObjectURL = () => null;
  // const apiDefinition = jest
  //   .fn()
  //   .mockImplementation(() => Promise.resolve(axiosResponse));
  const apiDefinition = () => Promise.resolve(axiosResponse);

  // const apiDefinition = jest.fn();
  const onBtnClick = jest.fn();
  const preExport = jest.fn();
  const postExport = jest.fn();
  const onError = jest.fn();
  // const container = document.createElement('div');
  // document.body.appendChild(container);
  // const dom = ReactDOM.createRoot(container)

  test('is initialized', async () => {
    const { result } = renderHook(() =>
      useExportFile({
        apiDefinition,
        preExport,
        postExport,
        onError,
      }),
    );

    const ref = result?.current?.ref;
    const url = result?.current?.url;
    // const a = React.createElement("a", {
    //   href: url,
    //   ref: ref,
    //   onClick: onBtnClick
    // });
    // dom.render(a);
    const container = render(<a role="anchor-btn" href={url} ref={ref} onClick={onBtnClick}></a>)
    // console.log(ref);
    const exportHandler = result?.current?.exportHandler;
    await act(async () => {
      await exportHandler();
      // console.log(url);
    });
    // console.log(result?.current);
    // console.log(result?.all);
    expect(typeof exportHandler).toBe('function');
    expect(result?.current?.ref?.current).toEqual(await container.findByRole("anchor-btn"))
    expect(result?.current?.url).toEqual(URL)
    expect(result?.current?.name).toEqual(FILE_NAME)
    expect(onBtnClick).toHaveBeenCalled();
    // expect(apiDefinition).toHaveBeenCalledTimes(1)
  });
});
