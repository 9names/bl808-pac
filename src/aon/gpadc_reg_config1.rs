#[doc = "Register `gpadc_reg_config1` reader"]
pub struct R(crate::R<GPADC_REG_CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_CONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_config1` writer"]
pub struct W(crate::W<GPADC_REG_CONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPADC_REG_CONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_CONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_cal_os_en` reader - "]
pub type GPADC_CAL_OS_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_cal_os_en` writer - "]
pub type GPADC_CAL_OS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_cont_conv_en` reader - "]
pub type GPADC_CONT_CONV_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_cont_conv_en` writer - "]
pub type GPADC_CONT_CONV_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_res_sel` reader - "]
pub type GPADC_RES_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_res_sel` writer - "]
pub type GPADC_RES_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_vcm_sel_en` reader - "]
pub type GPADC_VCM_SEL_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_vcm_sel_en` writer - "]
pub type GPADC_VCM_SEL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_vcm_hyst_sel` reader - "]
pub type GPADC_VCM_HYST_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_vcm_hyst_sel` writer - "]
pub type GPADC_VCM_HYST_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_lowv_det_en` reader - "]
pub type GPADC_LOWV_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_lowv_det_en` writer - "]
pub type GPADC_LOWV_DET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_pwm_trg_en` reader - "]
pub type GPADC_PWM_TRG_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_pwm_trg_en` writer - "]
pub type GPADC_PWM_TRG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_clk_ana_dly` reader - "]
pub type GPADC_CLK_ANA_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_clk_ana_dly` writer - "]
pub type GPADC_CLK_ANA_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `gpadc_clk_ana_dly_en` reader - "]
pub type GPADC_CLK_ANA_DLY_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_clk_ana_dly_en` writer - "]
pub type GPADC_CLK_ANA_DLY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_clk_ana_inv` reader - "]
pub type GPADC_CLK_ANA_INV_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_clk_ana_inv` writer - "]
pub type GPADC_CLK_ANA_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_clk_div_ratio` reader - "]
pub type GPADC_CLK_DIV_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_clk_div_ratio` writer - "]
pub type GPADC_CLK_DIV_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `gpadc_scan_length` reader - "]
pub type GPADC_SCAN_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_scan_length` writer - "]
pub type GPADC_SCAN_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `gpadc_scan_en` reader - "]
pub type GPADC_SCAN_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_scan_en` writer - "]
pub type GPADC_SCAN_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_dither_en` reader - "]
pub type GPADC_DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `gpadc_dither_en` writer - "]
pub type GPADC_DITHER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, bool, O>;
#[doc = "Field `gpadc_v11_sel` reader - "]
pub type GPADC_V11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_v11_sel` writer - "]
pub type GPADC_V11_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `gpadc_v18_sel` reader - "]
pub type GPADC_V18_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpadc_v18_sel` writer - "]
pub type GPADC_V18_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_CONFIG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&self) -> GPADC_CAL_OS_EN_R {
        GPADC_CAL_OS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&self) -> GPADC_CONT_CONV_EN_R {
        GPADC_CONT_CONV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&self) -> GPADC_RES_SEL_R {
        GPADC_RES_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_vcm_sel_en(&self) -> GPADC_VCM_SEL_EN_R {
        GPADC_VCM_SEL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_vcm_hyst_sel(&self) -> GPADC_VCM_HYST_SEL_R {
        GPADC_VCM_HYST_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_lowv_det_en(&self) -> GPADC_LOWV_DET_EN_R {
        GPADC_LOWV_DET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpadc_pwm_trg_en(&self) -> GPADC_PWM_TRG_EN_R {
        GPADC_PWM_TRG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gpadc_clk_ana_dly(&self) -> GPADC_CLK_ANA_DLY_R {
        GPADC_CLK_ANA_DLY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_clk_ana_dly_en(&self) -> GPADC_CLK_ANA_DLY_EN_R {
        GPADC_CLK_ANA_DLY_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&self) -> GPADC_CLK_ANA_INV_R {
        GPADC_CLK_ANA_INV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&self) -> GPADC_CLK_DIV_RATIO_R {
        GPADC_CLK_DIV_RATIO_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&self) -> GPADC_SCAN_LENGTH_R {
        GPADC_SCAN_LENGTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&self) -> GPADC_SCAN_EN_R {
        GPADC_SCAN_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&self) -> GPADC_DITHER_EN_R {
        GPADC_DITHER_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&self) -> GPADC_V11_SEL_R {
        GPADC_V11_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&self) -> GPADC_V18_SEL_R {
        GPADC_V18_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cal_os_en(&mut self) -> GPADC_CAL_OS_EN_W<0> {
        GPADC_CAL_OS_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cont_conv_en(&mut self) -> GPADC_CONT_CONV_EN_W<1> {
        GPADC_CONT_CONV_EN_W::new(self)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_res_sel(&mut self) -> GPADC_RES_SEL_W<2> {
        GPADC_RES_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vcm_sel_en(&mut self) -> GPADC_VCM_SEL_EN_W<8> {
        GPADC_VCM_SEL_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vcm_hyst_sel(&mut self) -> GPADC_VCM_HYST_SEL_W<9> {
        GPADC_VCM_HYST_SEL_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_lowv_det_en(&mut self) -> GPADC_LOWV_DET_EN_W<10> {
        GPADC_LOWV_DET_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pwm_trg_en(&mut self) -> GPADC_PWM_TRG_EN_W<11> {
        GPADC_PWM_TRG_EN_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_dly(&mut self) -> GPADC_CLK_ANA_DLY_W<12> {
        GPADC_CLK_ANA_DLY_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_dly_en(&mut self) -> GPADC_CLK_ANA_DLY_EN_W<16> {
        GPADC_CLK_ANA_DLY_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_inv(&mut self) -> GPADC_CLK_ANA_INV_W<17> {
        GPADC_CLK_ANA_INV_W::new(self)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_div_ratio(&mut self) -> GPADC_CLK_DIV_RATIO_W<18> {
        GPADC_CLK_DIV_RATIO_W::new(self)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_length(&mut self) -> GPADC_SCAN_LENGTH_W<21> {
        GPADC_SCAN_LENGTH_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_en(&mut self) -> GPADC_SCAN_EN_W<25> {
        GPADC_SCAN_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dither_en(&mut self) -> GPADC_DITHER_EN_W<26> {
        GPADC_DITHER_EN_W::new(self)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_v11_sel(&mut self) -> GPADC_V11_SEL_W<27> {
        GPADC_V11_SEL_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_v18_sel(&mut self) -> GPADC_V18_SEL_W<29> {
        GPADC_V18_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_config1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config1](index.html) module"]
pub struct GPADC_REG_CONFIG1_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_config1::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config1::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_config1 to value 0x000c_0002"]
impl crate::Resettable for GPADC_REG_CONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_0002;
}
