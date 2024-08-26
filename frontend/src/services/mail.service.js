import { http } from '@/http';

const check = async (data) => {
  return await http.post("email", data).then(response => {
    return response;
  });
}

export const mailService = {
  check,
}
