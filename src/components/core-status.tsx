import { LoadingOutlined } from '@ant-design/icons';
import clsx from 'clsx';
import useStore from 'store';

const StatusMap = {
  Started: <div className={clsx('bg-green-500 rounded-full', 'w-4 h-4')}></div>,
  Restarting: <LoadingOutlined />,
  Stopped: <div className={clsx('bg-red-500 rounded-full', 'w-4 h-4')}></div>,
};

const CoreStatus = () => {
  const coreStatus = useStore((s) => s.venus.coreStatus);

  return (
    <>
      <div className="flex items-center">
        <div className="mr-1">{StatusMap[coreStatus]}</div>
        <div className="">Core 5.2</div>
      </div>
    </>
  );
};

export default CoreStatus;
