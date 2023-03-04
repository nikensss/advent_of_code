import { day1 } from './day1';
import { day2 } from './day2';
import { day3 } from './day3';

const main = async (): Promise<void> => {
  await day1();
  await day2();
  await day3();
};

main()
  .then(() => console.log('Done!'))
  .catch(console.error);
