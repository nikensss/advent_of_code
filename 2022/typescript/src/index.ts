import { day1 } from './day1';

const main = async (): Promise<void> => {
  await day1();
};

main()
  .then(() => console.log('Done!'))
  .catch(console.error);
