#[doc = "Register `gauge` reader"]
pub struct R(crate::R<GAUGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAUGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAUGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAUGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gauge` writer"]
pub struct W(crate::W<GAUGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAUGE_SPEC>;
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
impl From<crate::W<GAUGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAUGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gauge_reserve` reader - "]
pub type GAUGE_RESERVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_reserve` writer - "]
pub type GAUGE_RESERVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 3, O>;
#[doc = "Field `gauge_ictrl_adc` reader - "]
pub type GAUGE_ICTRL_ADC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_ictrl_adc` writer - "]
pub type GAUGE_ICTRL_ADC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 2, O>;
#[doc = "Field `gauge_dem_en` reader - "]
pub type GAUGE_DEM_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_dem_en` writer - "]
pub type GAUGE_DEM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_ckb_en` reader - "]
pub type GAUGE_CKB_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_ckb_en` writer - "]
pub type GAUGE_CKB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_chop_phas` reader - "]
pub type GAUGE_CHOP_PHAS_R = crate::BitReader<bool>;
#[doc = "Field `gauge_chop_phas` writer - "]
pub type GAUGE_CHOP_PHAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_chop_freq` reader - "]
pub type GAUGE_CHOP_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_chop_freq` writer - "]
pub type GAUGE_CHOP_FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 3, O>;
#[doc = "Field `gauge_chop_en` reader - "]
pub type GAUGE_CHOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_chop_en` writer - "]
pub type GAUGE_CHOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_sel_edge` reader - "]
pub type GAUGE_SEL_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `gauge_sel_edge` writer - "]
pub type GAUGE_SEL_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_quan_gain` reader - "]
pub type GAUGE_QUAN_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gauge_quan_gain` writer - "]
pub type GAUGE_QUAN_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 2, O>;
#[doc = "Field `gauge_sdm_pu` reader - "]
pub type GAUGE_SDM_PU_R = crate::BitReader<bool>;
#[doc = "Field `gauge_sdm_pu` writer - "]
pub type GAUGE_SDM_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_channel_sel` reader - "]
pub type GAUGE_CHANNEL_SEL_R = crate::BitReader<bool>;
#[doc = "Field `gauge_channel_sel` writer - "]
pub type GAUGE_CHANNEL_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_channel_en` reader - "]
pub type GAUGE_CHANNEL_EN_R = crate::BitReader<bool>;
#[doc = "Field `gauge_channel_en` writer - "]
pub type GAUGE_CHANNEL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_lp_mode` reader - "]
pub type GAUGE_LP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `gauge_lp_mode` writer - "]
pub type GAUGE_LP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `tmux_gauge_power` reader - "]
pub type TMUX_GAUGE_POWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux_gauge_power` writer - "]
pub type TMUX_GAUGE_POWER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 3, O>;
#[doc = "Field `ten_gauge_power` reader - "]
pub type TEN_GAUGE_POWER_R = crate::BitReader<bool>;
#[doc = "Field `ten_gauge_power` writer - "]
pub type TEN_GAUGE_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `ntc_bias_sel` reader - "]
pub type NTC_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ntc_bias_sel` writer - "]
pub type NTC_BIAS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUGE_SPEC, u8, u8, 4, O>;
#[doc = "Field `ntc_bias_en` reader - "]
pub type NTC_BIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `ntc_bias_en` writer - "]
pub type NTC_BIAS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_ldo_pu` reader - "]
pub type GAUGE_LDO_PU_R = crate::BitReader<bool>;
#[doc = "Field `gauge_ldo_pu` writer - "]
pub type GAUGE_LDO_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_vcm_pu` reader - "]
pub type GAUGE_VCM_PU_R = crate::BitReader<bool>;
#[doc = "Field `gauge_vcm_pu` writer - "]
pub type GAUGE_VCM_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
#[doc = "Field `gauge_bg_pu` reader - "]
pub type GAUGE_BG_PU_R = crate::BitReader<bool>;
#[doc = "Field `gauge_bg_pu` writer - "]
pub type GAUGE_BG_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAUGE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gauge_reserve(&self) -> GAUGE_RESERVE_R {
        GAUGE_RESERVE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gauge_ictrl_adc(&self) -> GAUGE_ICTRL_ADC_R {
        GAUGE_ICTRL_ADC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_dem_en(&self) -> GAUGE_DEM_EN_R {
        GAUGE_DEM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gauge_ckb_en(&self) -> GAUGE_CKB_EN_R {
        GAUGE_CKB_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gauge_chop_phas(&self) -> GAUGE_CHOP_PHAS_R {
        GAUGE_CHOP_PHAS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gauge_chop_freq(&self) -> GAUGE_CHOP_FREQ_R {
        GAUGE_CHOP_FREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gauge_chop_en(&self) -> GAUGE_CHOP_EN_R {
        GAUGE_CHOP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gauge_sel_edge(&self) -> GAUGE_SEL_EDGE_R {
        GAUGE_SEL_EDGE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn gauge_quan_gain(&self) -> GAUGE_QUAN_GAIN_R {
        GAUGE_QUAN_GAIN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gauge_sdm_pu(&self) -> GAUGE_SDM_PU_R {
        GAUGE_SDM_PU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gauge_channel_sel(&self) -> GAUGE_CHANNEL_SEL_R {
        GAUGE_CHANNEL_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gauge_channel_en(&self) -> GAUGE_CHANNEL_EN_R {
        GAUGE_CHANNEL_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gauge_lp_mode(&self) -> GAUGE_LP_MODE_R {
        GAUGE_LP_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tmux_gauge_power(&self) -> TMUX_GAUGE_POWER_R {
        TMUX_GAUGE_POWER_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_gauge_power(&self) -> TEN_GAUGE_POWER_R {
        TEN_GAUGE_POWER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ntc_bias_sel(&self) -> NTC_BIAS_SEL_R {
        NTC_BIAS_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ntc_bias_en(&self) -> NTC_BIAS_EN_R {
        NTC_BIAS_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gauge_ldo_pu(&self) -> GAUGE_LDO_PU_R {
        GAUGE_LDO_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gauge_vcm_pu(&self) -> GAUGE_VCM_PU_R {
        GAUGE_VCM_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gauge_bg_pu(&self) -> GAUGE_BG_PU_R {
        GAUGE_BG_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_reserve(&mut self) -> GAUGE_RESERVE_W<0> {
        GAUGE_RESERVE_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_ictrl_adc(&mut self) -> GAUGE_ICTRL_ADC_W<3> {
        GAUGE_ICTRL_ADC_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_dem_en(&mut self) -> GAUGE_DEM_EN_W<5> {
        GAUGE_DEM_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_ckb_en(&mut self) -> GAUGE_CKB_EN_W<6> {
        GAUGE_CKB_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_chop_phas(&mut self) -> GAUGE_CHOP_PHAS_W<7> {
        GAUGE_CHOP_PHAS_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_chop_freq(&mut self) -> GAUGE_CHOP_FREQ_W<8> {
        GAUGE_CHOP_FREQ_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_chop_en(&mut self) -> GAUGE_CHOP_EN_W<11> {
        GAUGE_CHOP_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_sel_edge(&mut self) -> GAUGE_SEL_EDGE_W<12> {
        GAUGE_SEL_EDGE_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_quan_gain(&mut self) -> GAUGE_QUAN_GAIN_W<13> {
        GAUGE_QUAN_GAIN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_sdm_pu(&mut self) -> GAUGE_SDM_PU_W<15> {
        GAUGE_SDM_PU_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_channel_sel(&mut self) -> GAUGE_CHANNEL_SEL_W<16> {
        GAUGE_CHANNEL_SEL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_channel_en(&mut self) -> GAUGE_CHANNEL_EN_W<17> {
        GAUGE_CHANNEL_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_lp_mode(&mut self) -> GAUGE_LP_MODE_W<18> {
        GAUGE_LP_MODE_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_gauge_power(&mut self) -> TMUX_GAUGE_POWER_W<20> {
        TMUX_GAUGE_POWER_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ten_gauge_power(&mut self) -> TEN_GAUGE_POWER_W<23> {
        TEN_GAUGE_POWER_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ntc_bias_sel(&mut self) -> NTC_BIAS_SEL_W<24> {
        NTC_BIAS_SEL_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ntc_bias_en(&mut self) -> NTC_BIAS_EN_W<28> {
        NTC_BIAS_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_ldo_pu(&mut self) -> GAUGE_LDO_PU_W<29> {
        GAUGE_LDO_PU_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_vcm_pu(&mut self) -> GAUGE_VCM_PU_W<30> {
        GAUGE_VCM_PU_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gauge_bg_pu(&mut self) -> GAUGE_BG_PU_W<31> {
        GAUGE_BG_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gauge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gauge](index.html) module"]
pub struct GAUGE_SPEC;
impl crate::RegisterSpec for GAUGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gauge::R](R) reader structure"]
impl crate::Readable for GAUGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gauge::W](W) writer structure"]
impl crate::Writable for GAUGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gauge to value 0x0800_29a8"]
impl crate::Resettable for GAUGE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_29a8;
}
