// import { render, fireEvent } from '@testing-library/svelte';
// import ConnectForm from '../routes/(home)/+page.svelte';
// import { invoke } from '@tauri-apps/api/tauri';

// // 使用 Vitest 的 Mock 功能
// vi.mock('@tauri-apps/api/tauri', () => ({
//   invoke: vi.fn()
// }));

// describe('ConnectForm Component', () => {
//   beforeEach(() => {
//     // 每次测试前重置 Mock
//     vi.clearAllMocks();
//   });

//   it('should render form elements correctly', () => {
//     const { getByLabelText, getByText } = render(ConnectForm);
//     expect(getByLabelText('Host:')).toBeInTheDocument();
//     expect(getByLabelText('Port:')).toBeInTheDocument();
//     expect(getByLabelText('Password:')).toBeInTheDocument();
//     expect(getByText('Connect')).toBeInTheDocument();
//   });

//   it('should call Tauri invoke on form submission', async () => {
//     const { getByLabelText, getByText } = render(ConnectForm);

//     const hostInput = getByLabelText('Host:');
//     const portInput = getByLabelText('Port:');
//     const passwordInput = getByLabelText('Password:');
//     const submitButton = getByText('Connect');

//     await fireEvent.input(hostInput, { target: { value: 'loca
