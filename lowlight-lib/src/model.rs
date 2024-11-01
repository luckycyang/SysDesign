// Generated from ONNX "src/model/test.onnx" by burn-import
use burn::nn::conv::Conv2d;
use burn::nn::conv::Conv2dConfig;
use burn::nn::PaddingConfig2d;
use burn::record::FullPrecisionSettings;
use burn::record::Recorder;
use burn::{
    module::Module,
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv2d1: Conv2d<B>,
    conv2d2: Conv2d<B>,
    conv2d3: Conv2d<B>,
    conv2d4: Conv2d<B>,
    conv2d5: Conv2d<B>,
    conv2d6: Conv2d<B>,
    conv2d7: Conv2d<B>,
    conv2d8: Conv2d<B>,
    conv2d9: Conv2d<B>,
    conv2d10: Conv2d<B>,
    conv2d11: Conv2d<B>,
    conv2d12: Conv2d<B>,
    conv2d13: Conv2d<B>,
    conv2d14: Conv2d<B>,
    phantom: core::marker::PhantomData<B>,
    device: burn::module::Ignored<B::Device>,
}

impl<B: Backend> Model<B> {
    pub fn from_file(file: &str, device: &B::Device) -> Self {
        let record = burn::record::NamedMpkFileRecorder::<FullPrecisionSettings>::new()
            .load(file.into(), device)
            .expect("Record file to exist.");
        Self::new(device).load_record(record)
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let conv2d1 = Conv2dConfig::new([3, 3], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(3)
            .with_bias(true)
            .init(device);
        let conv2d2 = Conv2dConfig::new([3, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d3 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(32)
            .with_bias(true)
            .init(device);
        let conv2d4 = Conv2dConfig::new([32, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d5 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(32)
            .with_bias(true)
            .init(device);
        let conv2d6 = Conv2dConfig::new([32, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d7 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(32)
            .with_bias(true)
            .init(device);
        let conv2d8 = Conv2dConfig::new([32, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d9 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(64)
            .with_bias(true)
            .init(device);
        let conv2d10 = Conv2dConfig::new([64, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d11 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(64)
            .with_bias(true)
            .init(device);
        let conv2d12 = Conv2dConfig::new([64, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d13 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(64)
            .with_bias(true)
            .init(device);
        let conv2d14 = Conv2dConfig::new([64, 3], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        Self {
            conv2d1,
            conv2d2,
            conv2d3,
            conv2d4,
            conv2d5,
            conv2d6,
            conv2d7,
            conv2d8,
            conv2d9,
            conv2d10,
            conv2d11,
            conv2d12,
            conv2d13,
            conv2d14,
            phantom: core::marker::PhantomData,
            device: burn::module::Ignored(device.clone()),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input1: Tensor<B, 4>) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let conv2d1_out1 = self.conv2d1.forward(input1.clone());
        let conv2d2_out1 = self.conv2d2.forward(conv2d1_out1);
        let relu1_out1 = burn::tensor::activation::relu(conv2d2_out1);
        let conv2d3_out1 = self.conv2d3.forward(relu1_out1.clone());
        let conv2d4_out1 = self.conv2d4.forward(conv2d3_out1);
        let relu2_out1 = burn::tensor::activation::relu(conv2d4_out1);
        let conv2d5_out1 = self.conv2d5.forward(relu2_out1.clone());
        let conv2d6_out1 = self.conv2d6.forward(conv2d5_out1);
        let relu3_out1 = burn::tensor::activation::relu(conv2d6_out1);
        let conv2d7_out1 = self.conv2d7.forward(relu3_out1.clone());
        let conv2d8_out1 = self.conv2d8.forward(conv2d7_out1);
        let relu4_out1 = burn::tensor::activation::relu(conv2d8_out1);
        let concat1_out1 = burn::tensor::Tensor::cat([relu3_out1, relu4_out1].into(), 1);
        let conv2d9_out1 = self.conv2d9.forward(concat1_out1);
        let conv2d10_out1 = self.conv2d10.forward(conv2d9_out1);
        let relu5_out1 = burn::tensor::activation::relu(conv2d10_out1);
        let concat2_out1 = burn::tensor::Tensor::cat([relu2_out1, relu5_out1].into(), 1);
        let conv2d11_out1 = self.conv2d11.forward(concat2_out1);
        let conv2d12_out1 = self.conv2d12.forward(conv2d11_out1);
        let relu6_out1 = burn::tensor::activation::relu(conv2d12_out1);
        let concat3_out1 = burn::tensor::Tensor::cat([relu1_out1, relu6_out1].into(), 1);
        let conv2d13_out1 = self.conv2d13.forward(concat3_out1);
        let conv2d14_out1 = self.conv2d14.forward(conv2d13_out1);
        let tanh1_out1 = burn::tensor::activation::tanh(conv2d14_out1);
        let constant1_out1: f32 = 2f32;
        let pow1_out1 = input1.clone().powf_scalar(constant1_out1);
        let sub1_out1 = pow1_out1.sub(input1.clone());
        let mul1_out1 = tanh1_out1.clone().mul(sub1_out1);
        let add1_out1 = input1.add(mul1_out1);
        let constant2_out1: f32 = 2f32;
        let pow2_out1 = add1_out1.clone().powf_scalar(constant2_out1);
        let sub2_out1 = pow2_out1.sub(add1_out1.clone());
        let mul2_out1 = tanh1_out1.clone().mul(sub2_out1);
        let add2_out1 = add1_out1.add(mul2_out1);
        let constant3_out1: f32 = 2f32;
        let pow3_out1 = add2_out1.clone().powf_scalar(constant3_out1);
        let sub3_out1 = pow3_out1.sub(add2_out1.clone());
        let mul3_out1 = tanh1_out1.clone().mul(sub3_out1);
        let add3_out1 = add2_out1.add(mul3_out1);
        let constant4_out1: f32 = 2f32;
        let pow4_out1 = add3_out1.clone().powf_scalar(constant4_out1);
        let sub4_out1 = pow4_out1.sub(add3_out1.clone());
        let mul4_out1 = tanh1_out1.clone().mul(sub4_out1);
        let add4_out1 = add3_out1.add(mul4_out1);
        let constant5_out1: f32 = 2f32;
        let pow5_out1 = add4_out1.clone().powf_scalar(constant5_out1);
        let sub5_out1 = pow5_out1.sub(add4_out1.clone());
        let mul5_out1 = tanh1_out1.clone().mul(sub5_out1);
        let add5_out1 = add4_out1.add(mul5_out1);
        let constant6_out1: f32 = 2f32;
        let pow6_out1 = add5_out1.clone().powf_scalar(constant6_out1);
        let sub6_out1 = pow6_out1.sub(add5_out1.clone());
        let mul6_out1 = tanh1_out1.clone().mul(sub6_out1);
        let add6_out1 = add5_out1.add(mul6_out1);
        let constant7_out1: f32 = 2f32;
        let pow7_out1 = add6_out1.clone().powf_scalar(constant7_out1);
        let sub7_out1 = pow7_out1.sub(add6_out1.clone());
        let mul7_out1 = tanh1_out1.clone().mul(sub7_out1);
        let add7_out1 = add6_out1.add(mul7_out1);
        let constant8_out1: f32 = 2f32;
        let pow8_out1 = add7_out1.clone().powf_scalar(constant8_out1);
        let sub8_out1 = pow8_out1.sub(add7_out1.clone());
        let mul8_out1 = tanh1_out1.clone().mul(sub8_out1);
        let add8_out1 = add7_out1.add(mul8_out1);
        (add8_out1, tanh1_out1)
    }
}
